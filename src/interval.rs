use std::time::Duration;
use std::mem;
use std::ffi::CString;
use libc;

unsafe extern "C" fn rust_caller<F: FnMut()>(a: *const libc::c_void) {
    let v: *mut F = mem::transmute(a);
    let v = &mut *v;
    v();
}

trait Interop {
    fn as_int(self, _: &mut Vec<CString>) -> libc::c_int;
}

impl Interop for i32 {
    fn as_int(self, _: &mut Vec<CString>) -> libc::c_int {
        self
    }
}

impl<'a> Interop for *const libc::c_void {
    fn as_int(self, _: &mut Vec<CString>) -> libc::c_int {
        self as libc::c_int
    }
}

pub fn create<F: FnMut() + 'static>(interval: Duration, f: F) {
    let b = Box::new(f);
    let a = Box::into_raw(b);
    let ms = interval.as_secs() as f64 * 1000.0 + interval.subsec_nanos() as f64 / 1_000_000.0;
    let ms = ms as i32;
    js! { (ms, a as *const libc::c_void,
           rust_caller::<F> as *const libc::c_void)
           b"\
           window.setInterval(function (e) {\
               Runtime.dynCall('vi', $2, [$1]);\
           }, $0);\
       \0" };
}
