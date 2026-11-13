#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Macro code first called after boot survives normal exit and a kill after the idle write."""
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time

binary = str(Path(sys.argv[1]).resolve())
with tempfile.TemporaryDirectory(prefix='late-macro-cache-') as tmp:
    root = Path(tmp)
    runner = root / 'relax'
    # The loader hard-links program executables, so the binary must share this filesystem.
    shutil.copy2(binary, runner)
    pkg = root / 'pkg'
    shutil.copytree(Path(__file__).parent / 'pkg', pkg, ignore=shutil.ignore_patterns('target', 'Cargo.lock'))
    source = pkg / 'src/main.rs'
    original = source.read_text()

    def launch(tag, generation, cache, idle=False):
        # Change only the app body: its macro crate and helper keep their content keys.
        source.write_text(original.replace('black_box(1u32)', f'black_box({generation}u32)'))
        (pkg / 'ready').unlink(missing_ok=True)
        env = dict(os.environ, RELAX_CACHE_DIR=str(root / cache), RELAX_VERBOSE='1',
                   RELAX_CACHE_TRACE=str(root / f'{tag}-{{pid}}.tsv'),
                   RELAX_AHEAD='minimal', RELAX_PANIC_PARK='0')
        path = root / f'{tag}.log'
        with path.open('w') as log:
            command = [str(runner), 'run', '--quiet', 'late-macro-cache-example']
            if idle:
                command += ['--', 'idle']
            process = subprocess.Popen(command, cwd=pkg, env=env, stdout=log,
                                       stderr=log, start_new_session=True)
            try:
                if idle:
                    deadline = time.monotonic() + 120
                    while not ((pkg / 'ready').exists() and 'proc-macro mid write:' in path.read_text()):
                        assert process.poll() is None, path.read_text()
                        assert time.monotonic() < deadline, f'late macro code never reached an idle write: {path.read_text()}'
                        time.sleep(0.05)
                    assert (pkg / 'ready').read_text() == '42'
                    os.killpg(process.pid, signal.SIGKILL)
                    assert process.wait(timeout=10) == -signal.SIGKILL
                    return []
                code = process.wait(timeout=120)
                log.flush()
                output = path.read_text()
                assert code == 0 and 'answer 42' in output, output
                traces = list(root.glob(f'{tag}-*.tsv'))
                assert traces, f'normal exit did not write a cache trace: {output}'
                rows = [line.split('\t') for trace in traces for line in trace.read_text().splitlines()]
                helper = [row for row in rows if len(row) == 8 and row[5] == 'late_macro::late_helper']
                assert helper, 'the runtime module did not call its macro helper'
                return helper
            except BaseException:
                log.flush()
                print(path.read_text(), file=sys.stderr)
                raise
            finally:
                if process.poll() is None:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()

    def hit(rows):
        reused = [row for row in rows if row[1] in ('hit', 'hit1')]
        assert reused, f'late macro code was lost; next run did not reuse it: {rows}'
        assert not any(row[1] == 'miss' for row in rows), rows
        return reused[0][6]

    cold = launch('exit-cold', 1, 'exit-cache')
    missed = [row for row in cold if row[1] == 'miss']
    assert missed, f'first run never compiled the helper: {cold}'
    assert hit(launch('exit-warm', 2, 'exit-cache')) == missed[0][6], 'helper key changed'
    launch('kill-cold', 3, 'kill-cache', idle=True)
    hit(launch('kill-warm', 4, 'kill-cache'))
    print('late_macro_cache: PASS (late helper persisted at exit and idle write, then reused)')
