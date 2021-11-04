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
            println!("X11");
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
            println!("Wayland");
            pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
            pd.nwh = data.display; // same as window, on wayland there ins't a concept of windows
        }

        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(data) => {
            return data.window;
        }
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(data) => {
            pd.nwh = data.window;
        }
        _ => panic!("Unsupported Window Manager"),
    }

    return pd;
}

fn get_render_type() -> RendererType {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return RendererType::Vulkan;
    #[cfg(target_os = "macos")]
    return RenderType::Metal;
}

fn main() {
    println!("GLFW");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Error initializing library");

    println!("Hello");
    let (mut window, events) = glfw
        .create_window(1080 as _, 900 as _, "Window 1", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (mut window2, events2) = glfw
        .create_window(1080 as _, 900 as _, "Window 2", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    println!("0");

    window.set_pos(200, 200);
    window.set_size_polling(true);

    window2.set_pos(1080 + 300, 200);
    window2.set_size_polling(true);

    window.focus();

    window.set_key_polling(true);
    window2.set_key_polling(true);
    // window2.set_key_polling(true);
    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.height = 0;
    init.resolution.width = 0;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = get_platform_data(&window);

    println!("Got platform data");

    let before = std::time::SystemTime::now();
    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }
    let after = std::time::SystemTime::now();
    println!(
        "Initialized library in {:?} seconds",
        after.duration_since(before).unwrap()
    );

    let mut framebuffer = bgfx::create_frame_buffer_from_nwh(
        get_platform_data(&window).nwh as *mut c_void,
        window.get_size().0 as u16,
        window.get_size().1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    );

    let mut framebuffer2 = bgfx::create_frame_buffer_from_nwh(
        get_platform_data(&window2).nwh as *mut c_void,
        window2.get_size().0 as u16,
        window2.get_size().1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    );
    println!("2");

    let windows = [window, window2];

    let mut should_close = false;
    while !should_close {
        glfw.poll_events();
        // first window
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }

            if let WindowEvent::Size(_, _) = event {
                let window = &windows[0];

                framebuffer = bgfx::create_frame_buffer_from_nwh(
                    get_platform_data(&window).nwh as *mut c_void,
                    window.get_size().0 as u16,
                    window.get_size().1 as u16,
                    CreateFrameBufferFromNwhArgs::default(),
                );
            }
        }

        // second window
        for (_, event) in glfw::flush_messages(&events2) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }

            if let WindowEvent::Size(width, height) = event {
                let window = &windows[1];

                framebuffer2 = bgfx::create_frame_buffer_from_nwh(
                    get_platform_data(&window).nwh as *mut c_void,
                    width as u16,
                    height as u16,
                    CreateFrameBufferFromNwhArgs::default(),
                );
            }
        }

        let mut idx = 0;

        for window in windows.iter() {
            let id: u16 = idx;
            let color = if idx & 1 == 0 { 0x103030ff } else { 0x755413ff };

            if id == 0 {
                bgfx::set_view_frame_buffer(id, &framebuffer);
            }
            else {
                bgfx::set_view_frame_buffer(id, &framebuffer2);
            }
            let size = window.get_framebuffer_size();

            // bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
            bgfx::set_view_rect(id, 0, 0, size.0 as _, size.1 as _);
            bgfx::set_view_clear(
                id,
                ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
                SetViewClearArgs {
                    rgba: color,
                    depth: 1.0,
                    stencil: 0,
                },
            );

            bgfx::touch(id);
            idx += 1;
        }

        bgfx::frame(false);
    }

    println!("Before exit");
    bgfx::shutdown();
    // unsafe{ glfwTerminate() };
    println!("after exit");
}