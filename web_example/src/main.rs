use bgfx::*;
use bgfx_rs::bgfx;
use glam::{EulerRot, Mat4, Vec3};
use glfw::Window;
mod mainloop;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

static CANVAS_ID: &[u8; 7] = b"canvas\0";

static VS_CUBES: &[u8] =
    include_bytes!("../../resources/examples/runtime/shaders/essl/vs_cubes.bin");
static FS_CUBES: &[u8] =
    include_bytes!("../../resources/examples/runtime/shaders/essl/fs_cubes.bin");

#[rustfmt::skip]
static CUBE_VERTICES: [PosColorVertex; 8] = [
    PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
    PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
];

#[rustfmt::skip]
static CUBE_INDICES: [u16; 36] = [
    0, 1, 2, // 0
    1, 3, 2,
    4, 6, 5, // 2
    5, 6, 7,
    0, 2, 4, // 4
    4, 2, 6,
    1, 5, 3, // 6
    5, 7, 3,
    0, 4, 1, // 8
    4, 5, 1,
    2, 3, 6, // 10
    6, 3, 7,
];

extern "C" {
    fn emscripten_performance_now() -> f64;
}

fn get_time() -> f32 {
    unsafe { emscripten_performance_now() as f32 * 1.0 / 1000.0 }
}

fn get_platform_data(_window: &Window) -> PlatformData {
    let mut pd = PlatformData::new();
    pd.nwh = CANVAS_ID.as_ptr() as *mut _;
    pd
}

fn create_shader_program() -> std::io::Result<Program> {
    let vs_data = Memory::copy(&VS_CUBES);
    let ps_data = Memory::copy(&FS_CUBES);

    let vs_shader = bgfx::create_shader(&vs_data);
    let ps_shader = bgfx::create_shader(&ps_data);

    Ok(bgfx::create_program(&vs_shader, &ps_shader, false))
}

struct State {
    vbh: VertexBuffer,
    ibh: IndexBuffer,
    shader_program: Program,
    glfw: glfw::Glfw,
}

fn main_callback(state: &mut State) {
    state.glfw.poll_events();

    let render_state = (StateWriteFlags::R
        | StateWriteFlags::G
        | StateWriteFlags::B
        | StateWriteFlags::A
        | StateWriteFlags::Z)
        .bits()
        | StateDepthTestFlags::LESS.bits()
        | StateCullFlags::CW.bits();

    let at = Vec3::new(0.0, 0.0, 0.0);
    let eye = Vec3::new(0.0, 0.0, -35.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let t = get_time();

    let aspect = WIDTH as f32 / HEIGHT as f32;

    let persp = Mat4::perspective_lh(60.0 * (std::f32::consts::PI / 180.0), aspect, 0.1, 100.0);
    let view = Mat4::look_at_lh(eye, at, up);

    bgfx::set_view_rect(0, 0, 0, WIDTH as _, HEIGHT as _);
    bgfx::touch(0);

    bgfx::set_view_transform(0, &view.to_cols_array(), &persp.to_cols_array());

    for yy in 0..11 {
        for xx in 0..11 {
            let x = -15.0 + (xx as f32) * 3.0;
            let y = -15.0 + (yy as f32) * 3.0;
            let xr = t + (xx as f32) * 0.21;
            let yr = t + (yy as f32) * 0.37;

            let rot = Mat4::from_euler(EulerRot::XYZ, xr, yr, 0.0);
            let transform = Mat4::from_translation(Vec3::new(x, y, 0.0)) * rot;

            bgfx::set_transform(&transform.to_cols_array(), 1);
            bgfx::set_vertex_buffer(0, &state.vbh, 0, std::u32::MAX);
            bgfx::set_index_buffer(&state.ibh, 0, std::u32::MAX);

            bgfx::set_state(render_state, 0);
            bgfx::submit(0, &state.shader_program, SubmitArgs::default());
        }
    }

    bgfx::frame(false);
}

fn main() -> std::io::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, _events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "cubes.rs bgfx-rs example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let mut init = Init::new();

    init.type_r = RendererType::OpenGLES;
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

    let layout = VertexLayoutBuilder::begin(RendererType::Noop)
        .add(Attrib::Position, 3, AttribType::Float, AddArgs::default())
        .add(
            Attrib::Color0,
            4,
            AttribType::Uint8,
            AddArgs {
                normalized: true,
                as_int: false,
            },
        )
        .end();

    let verts_mem = unsafe { Memory::reference(&CUBE_VERTICES) };
    let index_mem = unsafe { Memory::reference(&CUBE_INDICES) };

    let vbh = bgfx::create_vertex_buffer(&verts_mem, &layout, BufferFlags::NONE.bits());
    let ibh = bgfx::create_index_buffer(&index_mem, BufferFlags::NONE.bits());

    let shader_program = create_shader_program()?;

    bgfx::reset(WIDTH as _, HEIGHT as _, ResetArgs::default());
    bgfx::set_view_rect(0, 0, 0, WIDTH as _, HEIGHT as _);

    let state = Box::new(State {
        vbh,
        ibh,
        shader_program,
        glfw,
    });

    mainloop::run(main_callback, state);

    Ok(())
}
