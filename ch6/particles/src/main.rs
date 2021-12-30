use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::time::Instant;

struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(),
            height: height,
            width: width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);
}
