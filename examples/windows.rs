use bgfx::*;
use bgfx_rs::bgfx;
use core::ffi::c_void;
use glfw::{Action, Key, Window, WindowEvent};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

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
            pd.nwh = data.window as *mut c_void;
            pd.ndt = data.display as *mut c_void;
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

fn get_render_type() -> RendererType {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return RendererType::Vulkan;
    #[cfg(target_os = "macos")]
    return RendererType::Metal;
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Error initializing library");
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(1080 as _, 900 as _, "Window 1", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (mut window2, _events2) = glfw
        .create_window(1080 as _, 900 as _, "Window 2", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_pos(200, 200);
    window.set_size_polling(true);

    window2.set_pos(1080 + 300, 200);
    window2.set_size_polling(true);

    window.focus();

    window.set_key_polling(true);
    window2.set_key_polling(true);

    let mut init = Init::new();
    init.type_r = get_render_type();
    init.resolution.height = 0;
    init.resolution.width = 0;
    init.resolution.reset = ResetFlags::VSYNC.bits(); // this makes the window recreation smoth
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    {

    let windows = [window, window2];
    let mut framebuffers = [None, None];
    let mut frame_sizes = [(0, 0), (0, 0)];

    let mut should_close = false;
    while !should_close {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }
        }

        for idx in 0..2 {
            let window = &windows[idx];
            let size = window.get_framebuffer_size();

            if framebuffers[idx].is_none() || frame_sizes[idx] != size {
                framebuffers[idx] = Some(bgfx::create_frame_buffer_from_nwh(
                    get_platform_data(&window).nwh as *mut c_void,
                    size.0 as u16,
                    size.1 as u16,
                    CreateFrameBufferFromNwhArgs::default(),
                ));

                frame_sizes[idx] = size;
            }

            if let Some(frame_buffer) = &framebuffers[idx] {
                bgfx::set_view_frame_buffer(idx as _, &frame_buffer);
            }

            let color = if idx & 1 == 0 { 0x103030ff } else { 0x755413ff };

            bgfx::set_view_rect(idx as _, 0, 0, size.0 as _, size.1 as _);
            bgfx::set_view_clear(
                idx as _,
                ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
                SetViewClearArgs {
                    rgba: color,
                    depth: 1.0,
                    stencil: 0,
                },
            );

            bgfx::touch(idx as _);
        }

        bgfx::frame(false);
    }

    }

    bgfx::shutdown();
}
