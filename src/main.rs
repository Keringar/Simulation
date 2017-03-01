#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate nalgebra;

use gfx::traits::FactoryExt;
use gfx::Device;

mod resource;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub fn main() {
    let builder = glutin::WindowBuilder::new()
                    .with_title("Triangle example".to_string())
                    .with_dimensions(1024, 768)
                    .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) = 
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    //Combine vertex and fragment shaders
    let shader_set = factory.create_shader_set(include_bytes!("shaders/triangle.vert"), include_bytes!("shaders/triangle.frag")).unwrap();
    //Create new rasterizer that culls backfaces
    let mut fillmode = gfx::state::Rasterizer::new_fill().with_cull_back();
    //Set raster method, use a line of width 1
    fillmode.method = gfx::state::RasterMethod::Line(1);
    //Create pso with shader program, every three vertices is a new triangle and wireframe
    let pso = factory.create_pipeline_state(&shader_set, gfx::Primitive::TriangleStrip, fillmode, pipe::new()).unwrap();

    //Create encoder to translate to raw gl calls
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    const TRIANGLE: [Vertex; 4] = [
        Vertex { pos: [ -0.5, -0.5, 0.0 ] },
        Vertex { pos: [  0.5, -0.5, 0.0 ] },
        Vertex { pos: [  0.0,  0.5, 0.0 ] },
        Vertex { pos: [  1.0,  0.5, 0.0 ] },
    ];

    let locals = Locals {
        model: nalgebra::Isometry3::new(nalgebra::Vector3::x(), nalgebra::zero()).to_homogeneous().into(),
        view: nalgebra::Isometry3::look_at_rh(&nalgebra::Point3::new(0.0, 0.0, 10.0), &nalgebra::Point3::new(0.0, 0.0, 0.0), &nalgebra::Vector3::y()).to_homogeneous().into(),
        proj: nalgebra::Perspective3::new(1024.0/768.0, 3.14/4.0, 0.1, 1000.0).unwrap().into(),
    };

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let local_buffer = factory.create_constant_buffer(1);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        locals: local_buffer,
        out: main_color,
    };

    let start_time = std::time::Instant::now();

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => break 'main,
                _ => { /* Do nothing */ }
            }
        }

        let elapsed_time = std::time::Instant::now().duration_since(start_time);
        let elapsed_time = elapsed_time.as_secs() as f32 + elapsed_time.subsec_nanos() as f32 / (1000000000.0);

        let locals = Locals {
            model: nalgebra::Isometry3::new(nalgebra::Vector3::x() * elapsed_time.sin(), nalgebra::zero()).to_homogeneous().into(),
            view: locals.view,
            proj: locals.proj,
        };

        encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
        encoder.clear(&data.out, [0.0 ,0.0, 0.0, 1.0]);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}