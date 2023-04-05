use isosurface::marching_cubes::MarchingCubes;
use isosurface::source::Source;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Vector3};
use kiss3d::window::Window;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let grid_dim = (26, 26, 26);
    let scalar_field = generate_scalar_field(grid_dim);

    let window_size = (1024, 768);
    let mut window =
        Window::new_with_size("Scientific Visualization", window_size.0, window_size.1);

    window.set_light(Light::StickToCamera);

    let iso_values = [0.2, 0.4, 0.6, 0.8];
    for &iso_value in iso_values.iter() {
        let mut mc = MarchingCubes::new(grid_dim.0);
        let mut vertices = vec![];
        let mut indices = vec![];
        mc.extract(&scalar_field, &mut vertices, &mut indices);

        let vertices = vertices
            .chunks(3)
            .map(|v| Point3::new(v[0], v[1], v[2]))
            .collect::<Vec<_>>();

        // generate a mutable mesh
        let mut mesh = kiss3d::resource::Mesh::new(vertices, indices, None, None, false);

        let color = custom_color(iso_value);
        let mut node = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));
        node.set_color(color.x, color.y, color.z);
    }

    while window.render() {}
}

struct ScalarField {
    data: Vec<f32>,
    dimensions: (usize, usize, usize),
}

impl Source for ScalarField {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        // return the value at point x y z
        let x = (x * self.dimensions.0 as f32) as usize;
        let y = (y * self.dimensions.1 as f32) as usize;
        let z = (z * self.dimensions.2 as f32) as usize;
        self.data[x
            + y * self.dimensions.0 as usize
            + z * self.dimensions.0 as usize * self.dimensions.1 as usize]
    }
}

fn generate_scalar_field(dim: (usize, usize, usize)) -> ScalarField {
    let mut rng = rand::thread_rng();
    let mut scalar_field = ScalarField {
        data: Vec::with_capacity(dim.0 * dim.1 * dim.2),
        dimensions: dim,
    };
    for _ in 0..dim.0 * dim.1 * dim.2 {
        scalar_field.data.push(rng.gen_range(0.0..=1.0));
    }
    scalar_field
}

fn custom_color(value: f32) -> Vector3<f32> {
    Vector3::new(1.0 - value, value, 0.0)
}
