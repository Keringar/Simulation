#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate nalgebra;

use gfx::traits::FactoryExt;
use gfx::Device;

mod resource;
mod window;
mod camera;

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
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub fn main() {
    let window = window::WindowBuilder::new()
                    .with_title("Triangle example".to_string())
                    .with_dimensions(1024, 768)
                    .with_vsync()
                    .build();
            
    let mut camera = camera::Camera::new(&window);

    let (mut device, mut factory, main_color, mut main_depth) = 
        gfx_window_glutin::init_existing::<ColorFormat, DepthFormat>(&window);

    //Combine vertex and fragment shaders
    let shader_set = factory.create_shader_set(include_bytes!("shaders/triangle.vert"), include_bytes!("shaders/triangle.frag")).unwrap();
    //Create new rasterizer that culls backfaces
    let mut fillmode = gfx::state::Rasterizer::new_fill();//.with_cull_back();
    //Set raster method, use a line of width 1
    fillmode.method = gfx::state::RasterMethod::Line(1);
    //Create pso with shader program, every three vertices is a new triangle and wireframe
    let pso = factory.create_pipeline_state(&shader_set, gfx::Primitive::TriangleList, fillmode, pipe::new()).unwrap();

    //Create encoder to translate to raw gl calls
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let vertex_buffer = create_terrain_clipmesh(4);
    
    let locals = Locals {
        model: nalgebra::Isometry3::new(nalgebra::Vector3::x() * 0.0, nalgebra::zero()).to_homogeneous().into(),
        view: camera.view_matrix,
        proj: camera.proj_matrix,
    };

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_buffer, ());
    let local_buffer = factory.create_constant_buffer(1);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        locals: local_buffer,
        out_color: main_color,
        out_depth: main_depth,
    };

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => break 'main,
                _ => { /* Do nothing */ }
            }
        }

        encoder.update_constant_buffer(&data.locals, &locals);
        encoder.clear(&data.out_color, [0.0 ,0.0, 0.0, 1.0]);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn create_terrain_clipmesh(size: u32) -> Vec<Vertex> {
    let mut vertex_buffer = Vec::<Vertex>::new();

    for x in 0..size {
        for z in 0..size {
            let vertices = generate_quad_mesh(x as f32, z as f32, 1.0);

            vertex_buffer.extend_from_slice(&vertices);
        }
    }

    vertex_buffer
}

fn generate_quad_mesh(x: f32, y: f32, size: f32) -> [Vertex; 6] {
    let bottom_left = Vertex {
        pos: [x, 0.0, y],
    };

    let bottom_right = Vertex {
        pos: [x + size, 0.0, y],
    };

    let top_left = Vertex {
        pos: [x, 0.0, y + size],
    };

    let top_right = Vertex {
        pos: [x + size, 0.0, y + size],
    };

    [bottom_left, top_right, top_left, bottom_left, bottom_right, top_right]
}
