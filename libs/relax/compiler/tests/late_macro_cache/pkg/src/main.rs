pub use late_macro::value;
const INITIAL: u32 = value!(0);

fn main() {
    std::hint::black_box(1u32);
    assert_eq!(INITIAL, 0);
    #[cfg(relax)]
    let answer = {
        let module = std::relax::Module::load_source(
            "late",
            "memory/late.rs",
            "pub fn answer() -> u32 { late_macro_cache_example::value!(1) }",
            &["late_macro_cache_example"],
        )
        .unwrap();
        module.get::<fn() -> u32>("answer").unwrap()()
    };
    #[cfg(not(relax))]
    let answer = value!(1);
    assert_eq!(answer, 42);
    println!("answer {answer}");
    if std::env::args().any(|arg| arg == "idle") {
        std::fs::write("ready", "42").unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
