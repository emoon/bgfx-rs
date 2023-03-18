use core::ffi::c_void;
use std::mem::transmute;

#[repr(C)]
struct WrappedMainData {
    user_data: *const c_void,
    func: *const c_void,
}

type EmCallbackFunc = unsafe extern "C" fn(user_data: *mut c_void);

unsafe extern "C" fn mainloop_trampoline_ud<T>(user_data: *mut c_void) {
    let wd: &WrappedMainData = transmute(user_data);
    let f: &&(dyn Fn(&mut T) + 'static) = transmute(wd.func);
    let data = wd.user_data as *mut T;
    f(&mut *data);
}

extern "C" {
    pub fn emscripten_set_main_loop_arg(
        func: EmCallbackFunc,
        data: *const c_void,
        fps: i32,
        simulate_infinite_loop: i32,
    );
}

pub fn run<'a, F, T>(func: F, data: Box<T>)
where
    F: Fn(&mut T) + 'a,
{
    // Having the data on the stack is safe as the mainloop only exits after the application is about to end
    let f: Box<Box<dyn Fn(&mut T) + 'a>> = Box::new(Box::new(func));
    let func = Box::into_raw(f) as *const _;

    let wrapped_data = Box::new(WrappedMainData {
        user_data: Box::into_raw(data) as *const _,
        func,
    });

    unsafe {
        emscripten_set_main_loop_arg(
            mainloop_trampoline_ud::<T>,
            Box::into_raw(wrapped_data) as *const _,
            0,
            1,
        );
    }
}
