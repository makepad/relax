# Macro code compiled after boot

```sh
cargo run --release --manifest-path libs/relax/compiler/tests/late_macro_cache/pkg/Cargo.toml
uv run libs/relax/compiler/tests/late_macro_cache/run.py target/release/relax
```

The native program prints `answer 42`. Under Relax it loads a module at runtime; that module
calls a local proc macro's helper, which initial loading did not call. The e2e proves a cache
miss on the first run and a hit for the same key on the next run after normal exit. A second
cache is filled by an idle run, killed only after the macro cache's batched write completes;
the next run must hit the helper too. Changing the app body between runs forces its macros to
expand again while keeping the macro crate unchanged. Both runs use the minimal ahead policy
to keep compilation on demand. This is a persistence test, with no performance threshold.
