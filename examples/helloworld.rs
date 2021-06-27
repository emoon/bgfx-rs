use bgfx_rs as bgfx;
use glfw::{Action, Context, Key, Window};
use raw_window_handle::HasRawWindowHandle;
use raw_window_handle::RawWindowHandle;
use bgfx::*;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[cfg(target_os = "linux")]
fn update_platform_handle(pd: &mut PlatformData, window: &Window) {
    match window.raw_window_handle() {
        RawWindowHandle::Xlib(x_data) => {
            pd.ndt = x_data.display;
            pd.nwh = x_data.window as *mut core::ffi::c_void;
        }
        _ => panic!("Unsupported window type"),
    }
}

#[cfg(target_os = "windows")]
fn update_platform_handle(pd: &mut PlatformData, window: &Window) {
    match window.raw_window_handle() {
        RawWindowHandle::Windows(data) => {
            pd.nwh = data.hwnd as *mut core::ffi::c_void;
        }
        _ => panic!("Unsupported window type"),
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "helloworld.rs bgfx-sys example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut pd = bgfx::PlatformData::new();
    update_platform_handle(&mut pd, &window);

    bgfx::set_platform_data(&pd);

    // TODO: run ctor inside new
    let mut init = Init::new();
    bgfx_rs::init_ctor(&init);

    init.type_r = RendererType::Count;
    init.resolution.width = WIDTH as u32;
    init.resolution.height = HEIGHT as u32;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = pd;

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    bgfx::set_debug(DebugFlags::TEXT.bits());
    bgfx::set_view_clear(0, ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
         SetViewClearArgs { rgba: 0x103030ff, .. Default::default() });

    let mut old_size = (0, 0);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }

        let size = window.get_framebuffer_size();

        if old_size != size {
            bgfx::reset(
                size.0 as _,
                size.1 as _,
                ResetArgs::default());
            old_size = size;
        }

        bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
        bgfx::touch(0);

        bgfx::dbg_text_clear(DbgTextClearArgs::default());

        bgfx::dbg_text(0, 1, 0x0f, "Color can be changed with ANSI \x1b[9;me\x1b[10;ms\x1b[11;mc\x1b[12;ma\x1b[13;mp\x1b[14;me\x1b[0m code too.");
        bgfx::dbg_text(80, 1, 0x0f, "\x1b[;0m    \x1b[;1m    \x1b[; 2m    \x1b[; 3m    \x1b[; 4m    \x1b[; 5m    \x1b[; 6m    \x1b[; 7m    \x1b[0m");
        bgfx::dbg_text(80, 2, 0x0f, "\x1b[;8m    \x1b[;9m    \x1b[;10m    \x1b[;11m    \x1b[;12m    \x1b[;13m    \x1b[;14m    \x1b[;15m    \x1b[0m");
        bgfx::dbg_text(
            0,
            4,
            0x3f,
            "Description: Initialization and debug text with bgfx-rs Rust API."
        );

        bgfx::frame(false);
    }

    bgfx::shutdown();
}

