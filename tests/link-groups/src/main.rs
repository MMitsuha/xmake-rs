extern crate libc;

extern "C" {
    fn gamma() -> libc::c_int;
}

fn main() {
    let output = unsafe { gamma() };
    assert!(output == 3, "3 != {}", output);
}
