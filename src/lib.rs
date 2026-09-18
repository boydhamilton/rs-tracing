pub mod scene;

pub use scene::colour::{Colour, write_colour};
pub use scene::vec3::Vec3;
pub use scene::ray::Ray;
pub use scene::hittable::{HitRecord, Hittable};
pub use scene::sphere::Sphere;
pub use scene::hittable_list::HittableList;
pub use scene::camera::Camera;
