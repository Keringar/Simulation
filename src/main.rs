#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
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

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/triangle.vert"),
        include_bytes!("shaders/triangle.frag"),
        pipe::new()
    ).unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    const TRIANGLE: [Vertex; 3] = [
        Vertex { pos: [ -0.5, -0.5, 0.0, 0.0 ], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [  0.5, -0.5, 0.0, 0.0 ], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [  0.0, -0.5, 0.0, 0.0 ], color: [0.0, 0.0, 1.0] },
    ];

    const TRANSFORM: Transform = Transform {
        transform: [[1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]]
    };

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let transform_buffer = factory.create_constant_buffer(1);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        transform: transform_buffer,
        out: main_color,
    };

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => break 'main,
                _ => { /* Do nothing */ }
            }
        }

        encoder.clear(&data.out, [0.0 ,0.0, 0.0, 0.0]);
        let _ = encoder.update_buffer(&data.transform, &[TRANSFORM], 0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}