//! Reference-to-reference deref coercion `as` casts (`&String as &str`, `&Arc::from("hi") as &str`,
//! `&mut String as &mut str`, `&Vec<u8> as &[_]`): `cast.rs` treats coercion casts as coercions
//! first, recording the deref steps and trailing borrow at the target type (and deferring casts
//! whose source or target referent still contains an open inference variable).
use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

struct Wrap<T>(T);

impl<T> Deref for Wrap<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn make_wrap<T: From<&'static str>>(s: &'static str) -> Wrap<T> {
    Wrap(T::from(s))
}

struct Track {
    text: String,
    shared_calls: Cell<u32>,
    mut_calls: u32,
}

impl Track {
    fn new(s: &str) -> Self {
        Self {
            text: s.to_string(),
            shared_calls: Cell::new(0),
            mut_calls: 0,
        }
    }
}

impl Deref for Track {
    type Target = str;
    fn deref(&self) -> &str {
        self.shared_calls.set(self.shared_calls.get() + 1);
        &self.text
    }
}

impl DerefMut for Track {
    fn deref_mut(&mut self) -> &mut str {
        self.mut_calls += 1;
        &mut self.text
    }
}

#[test]
fn shared_deref_coercion_casts() {
    let s = String::from("hello");
    let a: &str = &s as &str;
    assert_eq!(a, "hello");

    let sr = &s;
    let b: &str = &sr as &str;
    assert_eq!(b, "hello");

    let v = vec![10u8, 20, 30];
    let sl: &[u8] = &v as &[u8];
    assert_eq!(sl, &[10, 20, 30]);
}

#[test]
fn mutable_deref_coercion_casts_and_reborrow() {
    let mut s = String::from("abc");
    {
        let r: &mut String = &mut s;
        let ms: &mut str = r as &mut str;
        ms.make_ascii_uppercase();
        // `r as &mut str` reborrows `r`, so `r` remains usable afterwards.
        assert_eq!(r.len(), 3);
    }
    assert_eq!(s, "ABC");

    let mut v = vec![1u8, 2, 3];
    let msl: &mut [u8] = &mut v as &mut [u8];
    msl[1] = 9;
    assert_eq!(v, vec![1, 9, 3]);

    let shared: &str = &mut s as &str;
    assert_eq!(shared, "ABC");
}

#[test]
fn open_infer_referent_deref_coercion_casts() {
    let from_arc: &str = &Arc::from("arc-str") as &str;
    assert_eq!(from_arc, "arc-str");

    let from_box: &str = &Box::from("boxed") as &str;
    assert_eq!(from_box, "boxed");

    let v = vec![4u8, 5, 6];
    let sl: &[u8] = &v as &[_];
    assert_eq!(sl, &[4, 5, 6]);

    // Inner type of `Wrap<?T>` is resolved after the cast; the deferred cast derefs
    // `Wrap<String> -> String -> str` in two steps.
    let w = make_wrap("deferred");
    let d = &w as &str;
    assert_eq!(d, "deferred");
    let owned: Wrap<String> = w;
    assert_eq!(owned.0, "deferred");
}

#[test]
fn deref_vs_deref_mut_selection_on_mut_source() {
    let mut t = Track::new("hi");
    {
        let s: &str = &mut t as &str;
        assert_eq!(s, "hi");
    }
    assert_eq!(t.shared_calls.get(), 1);
    assert_eq!(t.mut_calls, 0);

    {
        let ms: &mut str = &mut t as &mut str;
        ms.make_ascii_uppercase();
    }
    assert_eq!(t.shared_calls.get(), 1);
    assert_eq!(t.mut_calls, 1);
    assert_eq!(t.text, "HI");

    // `Arc<str>` implements `Deref` only (no `DerefMut`); `&mut Arc<str> as &str` must use `Deref`.
    let mut arc: Arc<str> = Arc::from("shared-only");
    let s: &str = &mut arc as &str;
    assert_eq!(s, "shared-only");
}
