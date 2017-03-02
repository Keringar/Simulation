use nalgebra;
use window;
use std;

pub struct Camera<'a> {
    pub fov: f32,
    pub position: nalgebra::Point3<f32>,
    pub view_matrix: [[f32; 4]; 4],
    pub proj_matrix: [[f32; 4]; 4],
    render_target: &'a window::Window,
}

impl<'a> Camera<'a> {
    pub fn new(render_target: &window::Window) -> Camera {
        let mut default = Camera {
            fov: 90.0,
            position: nalgebra::Point3::new(3.0, 3.0, 3.0),
            view_matrix: nalgebra::Isometry3::look_at_rh(&nalgebra::Point3::origin(), &nalgebra::Point3::origin(), &nalgebra::Vector3::y()).to_homogeneous().into(),
            proj_matrix: nalgebra::Matrix4::identity().into(),
            render_target: render_target,
        };

        default.recalculate_proj_matrix();
        default.update_view();
        default
    }

    pub fn update_view(&mut self) {
        println!("{}", self.position);
        let target_position = self.position + nalgebra::Vector3::new(-3.0, -3.0, -3.0);
        println!("{}", target_position);
        self.view_matrix = nalgebra::Isometry3::look_at_rh(&self.position, &target_position, &nalgebra::Vector3::y()).to_homogeneous().into();
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.recalculate_proj_matrix();
    }

    fn recalculate_proj_matrix(&mut self) {
        let (width, height) = self.render_target.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let fov_in_radians = self.fov * 0.0174533; //Conversion factor for 1 degree to x radians
        self.proj_matrix = nalgebra::Perspective3::new(aspect_ratio, fov_in_radians, 0.1, 1000.0).unwrap().into();
    }
}