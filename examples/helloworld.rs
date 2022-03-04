use bgfx::*;
use bgfx_rs::bgfx;
use glfw::{Action, Key, Window};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn get_platform_data(window: &Window) -> PlatformData {
    let mut pd = PlatformData::new();

    match window.raw_window_handle() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(data) => {
            pd.nwh = data.window as *mut _;
            pd.ndt = data.display as *mut _;
        }
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(data) => {
            pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
            pd.nwh = data.display;
        }

        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(data) => {
            pd.nwh = data.ns_window;
        }
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(data) => {
            pd.nwh = data.hwnd;
        }
        _ => panic!("Unsupported Window Manager"),
    }

    return pd;
}

#[cfg(target_os = "linux")]
fn get_render_type() -> RendererType {
    RendererType::OpenGL
}

#[cfg(not(target_os = "linux"))]
fn get_render_type() -> RendererType {
    RendererType::Count
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "helloworld.rs bgfx-rs example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.width = WIDTH as u32;
    init.resolution.height = HEIGHT as u32;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    bgfx::set_debug(DebugFlags::TEXT.bits());
    bgfx::set_view_clear(
        0,
        ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
        SetViewClearArgs {
            rgba: 0x103030ff,
            ..Default::default()
        },
    );

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
            bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
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
            "Description: Initialization and debug text with bgfx-rs Rust API.",
        );

        bgfx::frame(false);
    }

    bgfx::shutdown();
}
