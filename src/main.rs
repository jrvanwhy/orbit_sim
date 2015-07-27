extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vec2,Vec3};
use kiss3d::window::Window;

fn calc_accel(pos: &Vec2<f64>) -> Vec2<f64> {
	*pos * (-1./na::dot(pos, pos))
}

fn main() {
	let mut window = Window::new("Simulation");
	window.set_framerate_limit(Some(60));
	window.add_sphere(0.2);
	let mut moon = window.add_sphere(0.1);

	let mut pos = Vec2::new(1., 0.);
	let mut vel = Vec2::new(0., 0.5);

	const DT: f64 = 0.05;

	while window.render() {
		vel = vel + calc_accel(&pos) * DT;
		pos = pos + vel * DT;

		moon.set_local_translation(Vec3::new(pos.x as f32, pos.y as f32, 0.));
	}
}
