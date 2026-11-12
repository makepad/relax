use core::ffi::CStr;

fn main() {
    let value: &CStr = pm_c_string_mac::hello!();
    println!("{}", value.to_str().unwrap());
}
