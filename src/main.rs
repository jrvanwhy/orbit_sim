extern crate nalgebra as na;
use na::Vec2;

struct OneBody2D {
	pos: Vec2<f64>,
	vel: Vec2<f64>,
	time: f64
}

impl OneBody2D {
	fn new(pos: &Vec2<f64>, vel: &Vec2<f64>) -> OneBody2D {
		OneBody2D { pos: *pos, vel: *vel, time: 0. }
	}

	fn calc_accel(&self) -> Vec2<f64> {
		self.pos * (-1./na::dot(&self.pos, &self.pos))
	}

	fn get_pos(&mut self, time: f64) -> Vec2<f64> {
		const DT: f64 = 0.001;

		while self.time + DT < time {
			self.vel = self.vel + self.calc_accel() * DT;
			self.pos = self.pos + self.vel * DT;
			self.time += DT;
		}

		self.pos + self.vel * (time - self.time)
	}
}

fn main() {
	let mut sim = OneBody2D::new(&Vec2::new(1., 0.), &Vec2::new(0., 0.5));

	for i in 0..100000 {
		let pos = sim.get_pos((i as f64) / 100.0);
		println!("{}, {}", pos.x, pos.y);
	}
}
