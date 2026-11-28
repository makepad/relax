//! The `audio` service streams the app's program output until the client closes the channel.
//! OPEN takes no arguments in version 1. Each DATA message is one contiguous run of stereo
//! frames, encoded as `u32 LE json_len | json | interleaved i16 LE (L, R)`.
//! The header carries `{first, rate, ch: 2, gen, t_ns}`. Frame positions start at zero and
//! advance through drops. A gap or device rate change starts a new generation, so a client
//! can reset its resampler and partial packets. The first frame's timestamp is in nanoseconds
//! on the app's monotonic clock, shared with the video capture timestamps.

pub const SERVICE: &str = "audio";
/// One DATA message's ceiling, for the client's `open_bounded`.
pub const MAX_MESSAGE: usize = 64 << 10;
/// Frames in one record at most (85 ms at 48 kHz). Longer runs are split.
pub const MAX_RECORD_FRAMES: usize = 4096;

/// A run of stereo frames with no gap inside.
#[derive(Debug, PartialEq, Eq)]
pub struct AudioRecord {
    pub first_frame: u64,
    pub rate: u32,
    pub gen: u32,
    pub t_ns: u64,
    pub pcm: Vec<i16>,
}

pub fn encode_audio_record(r: &AudioRecord) -> Vec<u8> {
    let header = format!(
        r#"{{"first":{},"rate":{},"ch":2,"gen":{},"t_ns":{}}}"#,
        r.first_frame, r.rate, r.gen, r.t_ns
    );
    let bytes: Vec<_> = r.pcm.iter().flat_map(|s| s.to_le_bytes()).collect();
    crate::frames::encode_record(&header, &bytes)
}

pub fn decode_audio_record(msg: &[u8]) -> Result<AudioRecord, String> {
    if msg.len() > MAX_MESSAGE {
        return Err("oversized audio message".into());
    }
    let (len, body) = msg.split_first_chunk::<4>().ok_or("missing audio header length")?;
    let (header, bytes) = body.split_at_checked(u32::from_le_bytes(*len) as usize)
        .ok_or("truncated audio header")?;
    if bytes.is_empty() || bytes.len() % 4 != 0 || bytes.len() / 4 > MAX_RECORD_FRAMES {
        return Err("invalid stereo PCM length".into());
    }

    // The v1 header is a flat object of unsigned integers. Parse the numbers directly:
    // both sample positions and nanoseconds have the full u64 range.
    let whitespace = [' ', '\t', '\r', '\n'];
    let header = std::str::from_utf8(header).map_err(|_| "audio header is not UTF-8")?;
    let members = header.trim_matches(whitespace).strip_prefix('{')
        .and_then(|s| s.strip_suffix('}')).ok_or("expected audio header object")?;
    let mut fields = std::collections::BTreeMap::new();
    for member in members.split(',') {
        let (key, value) = member.split_once(':').ok_or("expected audio header field")?;
        let makepad_strict_json::Value::Str(key) = makepad_strict_json::parse(key.as_bytes())? else {
            return Err("expected audio header field name".into());
        };
        let value = value.trim_matches(whitespace);
        if !value.bytes().all(|b| b.is_ascii_digit()) || (value.len() > 1 && value.starts_with('0')) {
            return Err(format!("invalid audio field {key}"));
        }
        let number: u64 = value.parse().map_err(|_| format!("invalid audio field {key}"))?;
        if fields.insert(key, number).is_some() {
            return Err("duplicate audio header field".into());
        }
    }
    let get = |key: &str| fields.get(key).copied().ok_or_else(|| format!("missing audio field {key}"));
    if get("ch")? != 2 {
        return Err("audio record must be stereo".into());
    }
    let rate = get("rate")?;
    if !(8_000..=192_000).contains(&rate) {
        return Err("audio rate out of range".into());
    }
    Ok(AudioRecord {
        first_frame: get("first")?,
        rate: rate as u32,
        gen: get("gen")?.try_into().map_err(|_| "audio generation out of range")?,
        t_ns: get("t_ns")?,
        pcm: bytes.chunks_exact(2).map(|b| i16::from_le_bytes([b[0], b[1]])).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frames::encode_record;

    #[test]
    fn records_round_trip_and_bad_records_are_errors() {
        let record = AudioRecord {
            first_frame: u64::MAX - 2, rate: 48_000, gen: u32::MAX, t_ns: u64::MAX,
            pcm: vec![0x1234, -2, i16::MIN, i16::MAX],
        };
        let msg = encode_audio_record(&record);
        assert_eq!(&msg[msg.len() - 8..], &[0x34, 0x12, 0xfe, 0xff, 0, 0x80, 0xff, 0x7f], "little-endian L/R samples");
        assert_eq!(decode_audio_record(&msg).unwrap(), record, "every header field and sample");

        let header = r#"{"first":0,"rate":48000,"ch":2,"gen":0,"t_ns":42}"#;
        for (from, to) in [("}", ""), ("}", ",}"), ("}", "} x"), ("48000", "048000"),
            ("48000", "7999"), ("48000", "192001"), ("48000", "4295015296"), ("2,", "1,"),
            ("\"gen\":0", "\"gen\":4294967296"), ("\"t_ns\":42", "\"other\":42")] {
            let bad = header.replace(from, to);
            assert!(decode_audio_record(&encode_record(&bad, &[0; 4])).is_err(), "invalid header {bad}");
        }
        for len in [0, 3, 6, (MAX_RECORD_FRAMES + 1) * 4] {
            assert!(decode_audio_record(&encode_record(header, &vec![0; len])).is_err(), "invalid PCM length {len}");
        }
        assert!(decode_audio_record(&[0xff; 4]).is_err(), "truncated header");
        assert!(decode_audio_record(&[0; 3]).is_err(), "truncated length field");
    }
}
