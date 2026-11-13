# Parallelism opportunities in Relax

Relax is a live compiler and JIT runtime for Rust. It runs a Cargo package from source without
rustc/LLVM/linker, compiles each function on first call, optimizes hot functions while the program
runs, and patches source edits into the running process. It is ~438k lines of Rust with no crates.io
dependencies and targets x86-64 and AArch64.

This note is an analysis of where the code is already parallel, where it is still serial, and what
concrete change would use more cores. Line references are to this repository at the time of writing
(`libs/relax/*`, `platform/*`). Items 1 and 5 are applied in this pull request; the rest are
proposals.

## Constraint: this repo is a projection

`AGENTS.md` says Relax is slaved to the `work` branch of `makepad/makepad`, that code changes belong
upstream first and are then carried here, and that a pull request here is ported into `work` and
carried back rather than merged here directly. Only `README.md`, `LICENSE`, `Cargo.toml`, `AGENTS.md`,
`.gitignore`, `.nojekyll`, `suggestions1.md` and the website files live only in this repository. So
each item below is a proposal for the upstream compiler; the code in this pull request is the same
diff that would be carried here once it lands on `work`.

## What is already parallel

Before proposing anything, the existing parallelism is worth naming, because the obvious hot paths
already use it and a proposal that ignores it would duplicate work or fight the scheduler.

- **Front-end parse pool.** `libs/relax/front/src/preload.rs` is "Parallel Rust source scans and
  parsing with ordered macro publication". `jobs()` (line 918) reads `RELAX_JOBS`, else
  `options::default_parse_jobs`, clamped to 1..32. Derive pre-expansion runs in parallel ahead of
  parsing (`front/src/builtins.rs`). Demand-reference evaluation is split into cost-ordered batches
  and run on `crate::pool::scope` workers (`front/src/rs_items.rs`).
- **Check.** `libs/relax/compiler/src/check.rs` runs body owners and impl soundness through
  `par_map` (line 780) on `jobs()` threads (line 644: `RELAX_JOBS`, else cores, at most 16). Each
  body runs its stage sequence (early lints, typeck, stability, body lints, unsafety, privacy,
  patterns, borrowck, reports) inside `body_errors`. `makepad selftest` compiles on the same thread
  count.
- **JIT.** `libs/relax/compiler/src/jit_queue.rs` is the single compile queue. Jobs carry a tier
  (`Ahead`, `O2`, `O3`, `Spec`, `Tick`, `Check`) and a class; admission is decided before an entry
  leaves the queue. Tier-ups run on idle workers capped by `budget::background_limit()` (line 440);
  the startup's look-ahead uses `startup_ahead()` = `cores - 2`, clamped to 1..4 (line 320).
- **Cache hashing.** `libs/relax/compiler/src/cache.rs` computes per-file check nodes and text names
  on scoped threads up to 8 (`check_nodes_of` line 306, `text_names` line 339).
- **CPU budget.** `platform/cpu_budget/src/lib.rs` is the process-wide permit gate shared by native
  and Relax callers. Classes `INTERACTIVE=0`, `VISIBLE=1`, `BACKGROUND=2`; roles `APP=0`, `JIT=1`,
  `HAL=2`. `limit = cores - 1`, `background_limit = (limit - 2).min(physical - 1).max(1)`
  (lines 151-155). A worker that wants to run in the background must call
  `makepad_cpu_budget::set_thread_class(BACKGROUND)` and go through the budget's admission.

The pattern for a new worker is therefore fixed: start it with `run_threads::spawn` /
`spawn_sized` (256 MB compiler stacks, `COMPILER_STACK` in `base/src/run_threads.rs`), give it a
budget class, and let the budget decide admission. A section that spawns several tasks inside an
already-admitted job uses `crate::pool::scope`, whose tasks inherit the scope's class.

## Opportunities

### 1. Background type check was one thread (`compiler/src/typeck_bg.rs`) -- applied here

The module doc used to say: "After the first present ... **one thread** at the compile workers' low
priority type-checks every body owner of the dependency crates". `start()` spawns one `run-typeck-bg`
thread (line 38); `run()` walked owners depth-first from `main` and then crate by crate, calling
`check(&f, d, ...)` for one `DefId` at a time. The bodies are independent: `check` only takes a
`front_read` snapshot and calls `tcx.typeck_unstored(prog, d)`, and `check.rs` already calls
`tcx.typeck` concurrently from up to 16 `par_map` threads, so the type-checker itself is safe to run
on several threads. `FRONT_RW` is an `RwLock`, so several readers may hold it at once.

The work is large: the `RUNNING` doc comment mentions "signatures and declarations of 63k checked
owners" for one devhub panel, and `STATS` counts owners and crates. One thread is a fraction of
`background_limit` (on an 8-core box, `background_limit` is 5).

Change, applied here: the depth-first-from-main walk stays on the owner thread (it is the priority
prefix), and the crate-by-crate bulk runs on up to `budget::background_limit()` workers that pop
owners from a shared index. `seen`/`stored` stay on the owner thread; `failed` becomes a small
mutex-guarded set shared by the workers, and `report()` is already called concurrently by the
`par_map` body checks (its write serializes on `REPORT_FILE`). The
preemption handling in `check` (retry on `preempt::is_preempted` / `OwnerBusy` / `NeedsLoad`) is
per-body and does not change; each worker still yields to the program's own builds at its body's
preempt points, so a burst of lazy compiles is not held behind the check. Owners are deduplicated
first: `body_owners` names each item once per def, and concurrent checks of the same body would only
race on its memo.

### 2. Gandalf, the background borrow checker, is one worker (`types/src/borrowck/lazy.rs`)

`enqueue()` starts one `gandalf` thread (line 422); `worker()` (line 631) pops one `Item` from a
rank-ordered queue and checks it. The queue ranks exist so a live edit's check (`RANK_EDIT`) and hot
code waiting for its verdict (`RANK_HOT`) jump ahead of the bulk of discovery and owner checks.
Items are keyed by `(source identity, DefId, epoch)` and are independent; the checker's own driver is
per-def.

Change: run several workers over the same queue, preserving the rank ordering for the urgent ranks.
The rank is already read from the queue head (`q.order.first()`); with N workers, each pick must
still prefer `RANK_EDIT`/`RANK_HOT` before `RANK_OWNER`/`RANK_DISCOVER`, so the latency property that
motivates the single worker (a person waits for a live edit's verdict) is preserved by reserving one
worker for urgent ranks or by letting every worker take urgent work first. The per-worker
`FrontSource` (the `rehomed`/`Parked` dance in `worker`) has to be per-thread; the `VERDICTS` map is
already behind a mutex, and `CHECKED`/`VIOLATIONS`/`CHECK_NS` are atomics. This needs the runtime
counters measured before and after, because the pacing (`QUIET_NS`, `pace`) is tuned for one worker
and would need to be re-tuned.

### 3. Live-reload recheck and recompile run on the owner thread (`compiler/src/reload.rs`)

`reload.rs` header: "The owner thread holds for the whole transaction (**compiling off the unit is
the next step**)." In `apply_locked` (line 4472) step 4 type-checks every changed body and every
live instance and borrow-checks them, then step 5 compiles the changed slots, const readers and the
callers that inlined them. Both stages iterate the change set serially.

Change: step 4 is the same per-body work `check.rs` already runs through `par_map`; a reload with
many changed bodies (a whole-file save) can recheck them on scoped threads. Step 5 is the compile
work the JIT queue exists for: enqueue the changed slots instead of compiling them inline, so the
`run-live` worker overlaps them with the running program. This is the item the upstream comment
already names as next, so it is the natural follow-up rather than a new direction. It is also the
riskiest: `apply_locked` mutates the unit and the versions are all-or-nothing, so the parallel stage
must not touch the unit except through the existing owner requests.

### 4. Ahead-of-call compilation is serialized on the owner (`compiler/src/run.rs:3281`)

`start_ahead` compiles the previous run's startup functions in order on one `run-ahead` worker, one
`jit::compile_slot` at a time. The comment states why: "Compiles serialize with lazy compiles on the
owner thread, so the worker overlaps with the program's own execution, not with its compiles." So
ahead compilation cannot use more cores until compilation can move off the unit.

Change: this is blocked by the same "compiling off the unit" step as item 3. Once a build no longer
needs an owner request per function, `start_ahead` can hand its list to the JIT queue (a `Job`
variant already exists for `Ahead` work) and let `startup_ahead()` workers run several builds at
once. Until then, the only safe gain is to overlap the owner-independent parts (lowering inputs,
cache-key lookups) across the list.

### 5. Front-image verification was one background thread (`compiler/src/cache.rs:206`) -- applied here

`verify_front_images` spawns one `run-cache-verify` thread that iterates every mapped front image and
checksums it (`st.verify_at(*i)`), collecting bad stores. Images are independent and the work is
CPU- and IO-bound; a cold run's images are read back whole.

Change, applied here: checksum the images on scoped threads (the same shape as `check_nodes_of`),
then remove the bad stores on the owner thread. `verify_at` is a read of an already-validated image
and `Store` is already `Sync` (it sits behind an `Arc` in a `Mutex`), so each worker only needs a
`&Store`. The worker count is `budget::background_limit()`.

### 6. Macro expansion is inherently ordered (`front/src/preload.rs`, `front/src/macros.rs`)

`preload.rs` says the loader does "the serial macro work that can overlap later scans", and
`macros.rs` describes the macro table as an append-only interner shared across snapshots, with
publication ordered so that a later file sees exactly what a sequential parse would. `rs_items.rs`
calls the merge "the serial merge". This is a real limit: macro visibility is order-dependent and a
parallel expansion would change what a file sees. It is worth stating as a known limit, not a
target, so effort goes to the items above instead.

### 7. Not a gap: the HAL app check (`compiler/src/check.rs:603`)

`start_app_check` runs the whole check on one `run-app-check` thread, but `check_prepared` inside it
already uses `par_map`, so it fans out to `jobs()` threads. This is one background thread that
*owns* a parallel phase, not a serial phase. No change needed.

## Suggested order of work

1. Item 1 (background type check): read-only, budget-bounded, largest single-threaded phase. Applied.
2. Item 2 (Gandalf workers): same shape, more ordering care because of the urgent ranks.
3. Item 5 (front-image verification): small and isolated. Applied.
4. Items 3 and 4 together: they share the "compiling off the unit" dependency and should be done as
   one upstream change.

Each item must go through `run_threads` and the CPU budget, and each should be measured against the
existing counters (`STATS` in `typeck_bg.rs`, `CHECKED`/`CHECK_NS` in `borrowck/lazy.rs`, the queue's
stats in `jit_queue.rs`) before and after, since the budget's `background_limit` is the real cap and
a change that ignores it will starve Visible work.
