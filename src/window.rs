use glutin;
use gfx_window_glutin;
use std;
use std::ops::Deref;

pub struct WindowBuilder {
    width: u32,
    height: u32,
    vsync: bool,
    title: std::string::String,
}

pub struct Window {
    window_handle: glutin::Window,
}

impl Window {
    pub fn get_dimensions(&self) -> (u32, u32) {
        let (width, height) = self.window_handle.get_inner_size().unwrap();

        (width * self.window_handle.hidpi_factor() as u32, height * self.window_handle.hidpi_factor() as u32)
    }
}

impl Deref for Window {
    type Target = glutin::Window;

    fn deref(&self) -> &glutin::Window {
        &self.window_handle
    }
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            width: 0,
            height: 0,
            vsync: true,
            title: "Uninitialized Window!".to_string(),
        }
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> WindowBuilder {
        self.width = width;
        self.height = height;

        self
    }

    pub fn with_vsync(mut self) -> WindowBuilder {
        self.vsync = true;

        self
    }

    pub fn with_title(mut self, title: std::string::String) -> WindowBuilder {
        self.title = title;

        self
    }

    pub fn build(mut self) -> Window {
        let window = glutin::WindowBuilder::new()
                        .with_title(self.title)
                        .with_dimensions(self.width, self.height)
                        .with_min_dimensions(800, 600)
                        .with_vsync()
                        .with_gl_profile(glutin::GlProfile::Core)
                        .build()
                        .unwrap();

        Window {
            window_handle: window,
        }
    }
}