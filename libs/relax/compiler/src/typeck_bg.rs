//! `makepad run`'s background type check. Type checking on the startup path is demand-driven (a
//! build checks the bodies it needs, the ahead helpers the ones its callers are about to need);
//! every body is still checked (the user's rule, 2026-11-02). After the first present (a headless
//! program: once main runs) the check workers, at the compile workers' low priority, type-check
//! every body owner of the dependency crates -- first what main reaches (the fns its checked
//! bodies name, depth first) on the owner thread, then every other owner, crate by crate, on the
//! background workers -- yielding while the program's threads compile. An error is reported at
//! once, as a compile error of code that has not run (stderr, the launcher's report, the run's
//! diagnostics), whether that code ever runs or not; the program
//! itself never runs unchecked code (a build checks its body first, jit.rs). A crate whose every
//! body is clean stores its verdict (cache.rs crate_check_key), so a later run checks only what
//! changed, as cargo builds a dependency once. Not checked here: the app's own crates (checked
//! whole before main, run.rs), proc-macro crates (they run in the macro host) and the sysroot
//! (shipped checked, as rustc ships it). The bodies checked here are not kept
//! (Tcx::typeck_unstored): the program's builds keep only the ones they use.

use crate::jit::{self, FrontShared, Unit};
use crate::program::{DefId, DefKind, Program};
use crate::typeck::{Body, Res};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// body owners checked, owners with errors, crates whose clean verdict was stored, cpu ns, wall ns
/// from its start to its end (0 while it runs)
pub static STATS: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];

/// The check is under way (started and not ended): it fills the type side for every body owner of
/// the dependency crates, so the settle rebuild waits for its end. Rebuilt before it, the devhub
/// panel's compiler arena grew back from 0.55 to 1.05 GB committed (352 MB live, against 81 MB when
/// the rebuild came after it): signatures and declarations of 63k checked owners.
static RUNNING: AtomicBool = AtomicBool::new(false);

pub fn running() -> bool {
    RUNNING.load(Ordering::Acquire)
}

/// Starts the background check of unit `up` (`root`: the program's crate; `main`: where the
/// reachability walk starts).
pub fn start(up: *mut Unit, root: u32, main: Option<DefId>) {
    if std::env::var("RELAX_BG_TYPECK").is_ok_and(|v| v == "0") { return; }
    let upn = up as usize;
    let _k = crate::zone_alloc::kept();
    RUNNING.store(true, Ordering::Release);
    // (typeck recurses deeply on big bodies: the check workers' stack)
    let spawned = crate::run_threads::spawn_sized("run-typeck-bg", 256 << 20, move || {
        let _z = crate::zone_alloc::compiler_thread();
        // ended however it ends (done, the run stopping, a panic)
        struct Ended;
        impl Drop for Ended {
            fn drop(&mut self) { RUNNING.store(false, Ordering::Release); }
        }
        let _ended = Ended;
        run(upn as *mut Unit, root, main);
    });
    if spawned.is_err() { RUNNING.store(false, Ordering::Release); }
}

/// The run stops, or quits: a crate's verdict is stored only once all of it is checked, and the
/// next launch checks what this one left
fn stopping() -> bool {
    crate::preempt::is_stopped() || crate::owner::exit_tid() != 0 || crate::jit::QUIT_SEEN.load(Ordering::Relaxed)
}

/// The crates this check covers -- not the app's (checked before main), not a proc-macro crate,
/// not the sysroot, and no clean verdict stored for it -- with their verdicts' cache keys.
fn todo_crates(u: &Unit, root: u32, skipped: &[u128]) -> (Vec<bool>, Vec<Option<u128>>) {
    use crate::vfs::canonical as canon;
    let std_dir = format!("{}/", canon(&crate::std_dir()).trim_end_matches('/'));
    let prog = &u.prog;
    (0..prog.crates.len() as u32).map(|k| {
        let c = &prog.crates[k as usize];
        if c.proc_macro || k == root || crate::borrowck::event::app_crate(prog, k)
            || matches!(prog.syms.str(c.name), "core" | "alloc" | "std" | "proc_macro")
            || canon(c.root_path()).starts_with(&std_dir) {
            return (false, None);
        }
        let Some(c) = u.cache.as_ref() else { return (true, None) };
        let c = unsafe { &mut *c.get() };
        match c.crate_check_key(k, skipped[k as usize], 0) {
            Some(key) if matches!(c.borrowck_stored(key), Some(crate::borrowck::lazy::StoredVerdict::Clean)) => (false, None),
            key => (true, key),
        }
    }).unzip()
}

enum Checked {
    /// the fns its body names (empty: it has errors, or it is no body owner)
    Done(Vec<DefId>),
    /// it waits on a file the lazy selection skipped: checked again once that is loaded
    Later,
    /// the run is ending
    Stop,
}

/// The crates with an error, shared by the phase-B workers (a crate's clean verdict is not stored
/// while one of its bodies failed). One insert per crate: the same crate's bodies fail together.
struct Failed(relax_session::sync::Mutex<HashSet<u32>>);
impl Failed {
    fn new() -> Self { Self(relax_session::sync::Mutex::new(HashSet::new())) }
    fn insert(&self, k: u32) { self.0.lock().unwrap_or_else(|e| e.into_inner()).insert(k); }
    fn contains(&self, k: u32) -> bool { self.0.lock().unwrap_or_else(|e| e.into_inner()).contains(&k) }
}

fn run(up: *mut Unit, root: u32, main: Option<DefId>) {
    while !jit::past_startup() {
        if stopping() { return; }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    makepad_cpu_budget::set_thread_class(makepad_cpu_budget::BACKGROUND);
    let (t0, w0) = (relax_session::clock::cpu_ns(), jit::wall_ns());
    let f = unsafe { (*up).front.clone() };
    // the crates' skipped files read here, the keys made on the owner (the cache is the owner's)
    let n = { let r = jit::front_read("typeck bg crates"); r.of(&f).0.crates.len() as u32 };
    let skipped_text: Vec<u128> = (0..n).map(crate::check::skipped_text_hash).collect();
    let upn = up as usize;
    let (todo, keys) = crate::owner::call(crate::owner::Class::Publish, || todo_crates(unsafe { &*(upn as *const Unit) }, root, &skipped_text));
    if !todo.contains(&true) { return finish(t0, w0); }
    let mut seen: HashSet<DefId> = HashSet::new();
    let failed = Failed::new();
    let mut stored: HashSet<u32> = HashSet::new();
    // what main reaches, depth first (named_fns' order)
    let mut stack: Vec<DefId> = main.into_iter().collect();
    while let Some(d) = stack.pop() {
        if !seen.insert(d) { continue; }
        match check(&f, d, &todo, &failed) {
            Checked::Done(named) => stack.extend(named.into_iter().rev().filter(|d| !seen.contains(d))),
            Checked::Later => { seen.remove(&d); }
            Checked::Stop => return,
        }
    }
    // every other owner; the files the lazy selection skipped are loaded by the completion
    // thread meanwhile (jit.rs start_completion): their owners are checked as they appear
    loop {
        let (owners, skipped) = {
            let r = jit::front_read("typeck bg owners");
            let (prog, _) = r.of(&f);
            let crates: Vec<u32> = (0..prog.crates.len() as u32).filter(|&k| todo.get(k as usize).copied().unwrap_or(false)).collect();
            // one entry per owner: an item's defs all name it, so the serial walk would check
            // the same owner once per def
            let mut uniq: HashSet<DefId> = HashSet::new();
            let owners: Vec<DefId> = crate::check::body_owners(prog, &crates).into_iter()
                .filter(|d| !seen.contains(d) && uniq.insert(*d)).collect();
            let skipped: HashSet<u32> = prog.skipped_files.keys().map(|&m| prog.mods[m as usize].krate).collect();
            (owners, skipped)
        };
        let mut pending: HashSet<u32> = HashSet::new();
        // every other owner, the bulk of a cold run's bodies, on its own worker: the reachability
        // walk above is the priority and stays serial. A worker yields to the program's own builds
        // at the body's preempt points, as the serial walk did, so a burst of lazy compiles is not
        // held behind the check.
        for &d in &owners { seen.insert(d); }
        let next = std::sync::atomic::AtomicUsize::new(0);
        let results: Vec<(DefId, Checked)> = std::thread::scope(|sc| {
            let workers = makepad_cpu_budget::background_limit().max(1) as usize;
            let mut hs = Vec::with_capacity(workers.min(owners.len()).max(1));
            for _ in 0..workers.min(owners.len()).max(1) {
                let (owners, next, f, todo, failed) = (&owners, &next, &f, &todo, &failed);
                hs.push(std::thread::Builder::new().name("run-typeck-bg-w".into()).stack_size(256 << 20).spawn_scoped(sc, move || {
                    let _z = crate::zone_alloc::compiler_thread();
                    makepad_cpu_budget::set_thread_class(makepad_cpu_budget::BACKGROUND);
                    let mut local: Vec<(DefId, Checked)> = Vec::new();
                    loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(&d) = owners.get(i) else { break };
                        let c = check(f, d, todo, failed);
                        let stop = matches!(c, Checked::Stop);
                        local.push((d, c));
                        if stop { break; }
                    }
                    local
                }).expect("run-typeck-bg worker"));
            }
            hs.into_iter().flat_map(|h| match h.join() { Ok(v) => v, Err(e) => std::panic::resume_unwind(e) }).collect()
        });
        for (d, c) in results {
            match c {
                Checked::Done(_) => {}
                Checked::Later => {
                    seen.remove(&d);
                    let r = jit::front_read("typeck bg later");
                    pending.insert(r.of(&f).0.def(d).krate);
                }
                Checked::Stop => return,
            }
        }
        // a crate whose every body is loaded and checked clean: its verdict goes to the cache with
        // the borrow checker's (the run's next cache write). Not once a live edit's versions are in
        // the tables (a refused or undone one's too): the bodies checked were not all the boot's,
        // which the keys name.
        let boot = crate::reload::program_edits() == 0;
        for k in 0..todo.len() as u32 {
            if boot && todo[k as usize] && !failed.contains(k) && !skipped.contains(&k) && !pending.contains(&k) && stored.insert(k) {
                if let Some(key) = keys[k as usize] { crate::borrowck::lazy::push_verdict_key(key, crate::borrowck::lazy::StoredVerdict::Clean); }
                STATS[2].fetch_add(1, Ordering::Relaxed);
            }
        }
        if skipped.is_empty() && pending.is_empty() { break; }
        // files remain skipped: wait for the completion thread to load more
        loop {
            if stopping() { return; }
            std::thread::sleep(std::time::Duration::from_millis(200));
            let r = jit::front_read("typeck bg skipped");
            let (prog, _) = r.of(&f);
            if prog.skipped_files.is_empty() || prog.skipped_files.keys().map(|&m| prog.mods[m as usize].krate).collect::<HashSet<u32>>() != skipped { break; }
        }
    }
    finish(t0, w0)
}

fn finish(t0: u64, w0: u64) {
    STATS[3].store(relax_session::clock::cpu_ns() - t0, Ordering::Relaxed);
    STATS[4].store((jit::wall_ns() - w0).max(1), Ordering::Relaxed);
    if crate::run::VERBOSE.load(Ordering::Relaxed) {
        eprintln!("relax: background type check done: {} body owners checked, {} with errors, {} crates' clean verdicts stored, cpu {:.0} ms over {:.1} s",
            STATS[0].load(Ordering::Relaxed), STATS[1].load(Ordering::Relaxed), STATS[2].load(Ordering::Relaxed),
            STATS[3].load(Ordering::Relaxed) as f64 / 1e6, STATS[4].load(Ordering::Relaxed) as f64 / 1e9);
    }
}

/// Type-checks body owner `d` of a crate this check covers and reports its errors; a body of any
/// other crate is only read when a build checked it already (the walk goes on through it).
fn check(f: &FrontShared, d: DefId, todo: &[bool], failed: &Failed) -> Checked {
    loop {
        if stopping() { return Checked::Stop; }
        // the program's own compiles go first (a burst of the UI thread's lazy compiles)
        while !jit::tier1_workers_ready() {
            if stopping() { return Checked::Stop; }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        let r = jit::front_read("typeck bg");
        let (prog, tcx) = r.of(f);
        if d.0 as usize >= prog.defs.len() || crate::borrowck::body::CheckedBody::item(prog, d).is_none() { return Checked::Done(Vec::new()); }
        let krate = prog.def(d).krate;
        if !todo.get(krate as usize).copied().unwrap_or(false) {
            return Checked::Done(tcx.typeck_peek(prog, d).filter(|b| b.errors.is_empty()).map(|b| named_fns(prog, &b)).unwrap_or_default());
        }
        let body = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _p = crate::preempt::Preemptible::on();
            crate::preempt::point();
            tcx.typeck_unstored(prog, d)
        }));
        let body = match body {
            Ok(b) => b,
            // a front writer waits (a load, a patch), or the owner was busy with an earlier request:
            // checked again after it
            Err(p) if crate::preempt::is_preempted(&*p) || p.is::<crate::preempt::OwnerBusy>() => {
                drop(r);
                std::thread::sleep(std::time::Duration::from_millis(5));
                continue;
            }
            Err(p) if p.is::<crate::preempt::NeedsLoad>() && !prog.skipped_files.is_empty() => return Checked::Later,
            Err(p) => {
                // the checker failed on a body: a makepad run bug, reported like a build's
                // internal error, and the crate's verdict is not stored
                crate::hot_failure::checker_panic(&prog.def_path(d), &jit::payload_text(&*p));
                let e = format!("internal compiler error while checking {} -- Relax bug, report it{}", prog.def_path(d), jit::take_ice_location());
                STATS[1].fetch_add(1, Ordering::Relaxed);
                failed.insert(krate);
                report(&[e]);
                return Checked::Done(Vec::new());
            }
        };
        STATS[0].fetch_add(1, Ordering::Relaxed);
        if body.errors.is_empty() { return Checked::Done(named_fns(prog, &body)); }
        // a name in a file the lazy selection skipped: checked again once it is loaded
        if !body.deferred_misses.is_empty() && !prog.skipped_files.is_empty() { return Checked::Later; }
        STATS[1].fetch_add(1, Ordering::Relaxed);
        failed.insert(krate);
        report(&body.errors);
        return Checked::Done(Vec::new());
    }
}

/// A body's errors: compile errors of code that has not run (it may never run).
fn report(errors: &[String]) {
    for e in errors {
        if crate::run_state::observed() { crate::run_state::diagnostic(crate::run::app_domain(), crate::diag::from_legacy(e, "typeck")); }
        crate::run::report("error", e);
        eprintln!("{}", e);
    }
}

/// The fns a checked body names: its paths, then its method calls, then its operators.
fn named_fns(prog: &Program, b: &Body) -> Vec<DefId> {
    let mut out: Vec<DefId> = b.res.values().filter_map(|r| match r {
        Res::Def(d) if matches!(prog.def(*d).kind, DefKind::Fn | DefKind::AssocFn) => Some(*d),
        _ => None,
    }).collect();
    out.extend(b.methods.values().chain(b.binops.values()).map(|m| m.def));
    out
}

/// What this module holds of a run, as a fresh process has it (`run_end`). Nothing of the run is left that could see it.
pub(crate) unsafe fn reset_statics() {
    use relax_session::reset_static as reset;
    reset(&STATS, [const { AtomicU64::new(0) }; 5]);
    reset(&RUNNING, AtomicBool::new(false));
}
