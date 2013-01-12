use libc::c_void;
use core::private::run_in_bare_thread;

extern {
    fn rust_dbg_call(cb: *u8, data: *c_void);
}

fn main() {
    unsafe {
        do run_in_bare_thread() {
            unsafe {
                let i = &100;
                rust_dbg_call(callback, cast::transmute(i));
            }
        }
    }
}

extern fn callback(data: *c_void) {
    unsafe {
        let data: *int = cast::transmute(data);
        assert *data == 100;
    }
}