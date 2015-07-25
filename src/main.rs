extern crate nalgebra as na;
use na::Vec3;

const EARTH_RADIUS: f64 = 6_371_000.;
const EARTH_GACCEL: f64 = 9.81;
const GRAV_K: f64 = -EARTH_GACCEL * EARTH_RADIUS * EARTH_RADIUS;

fn calc_accel(pos: &Vec3<f64>, _vel: &Vec3<f64>) -> Vec3<f64> {
	let r_sqr = na::dot(pos, pos);
	*pos * (GRAV_K / (r_sqr*r_sqr*r_sqr).sqrt())
}

fn ode_step(pos: &mut Vec3<f64>, vel: &mut Vec3<f64>) {
	const DT: f64 = 0.001;
	
	*vel = *vel + calc_accel(&pos, &vel) * DT;
	*pos = *pos + *vel * DT;
}

fn main() {
	let mut pos = Vec3::new(EARTH_RADIUS + 300_000f64, 0., 0.);
	let mut vel = Vec3::new(0., (-pos.x * calc_accel(&pos, &Vec3::new(0f64, 0., 0.)).x).sqrt(), 0.);

	while pos.y >= 0. {
		ode_step(&mut pos, &mut vel);
	}

	while pos.y < 0. {
		ode_step(&mut pos, &mut vel);
	}

	println!("Position: ({}, {}, {}).", pos.x, pos.y, pos.z);
}
