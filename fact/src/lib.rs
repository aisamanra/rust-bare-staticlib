#![crate_type="lib"]
#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern "C" fn fact(n: isize) -> isize {
    if n == 0 {
        1
    } else {
        n * fact(n-1)
    }
}
