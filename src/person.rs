use crate::collision_physics;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Person {
    id: u32,
    radius: f32,
    position: [f32; 2],
    mass: f32,
    velocity: [f32; 2],     // m/s
    acceleration: [f32; 2], // m/s^2
}

impl Person {
    pub fn new(radius: f32, position: [f32; 2], velocity: [f32; 2]) -> Person {
        Person {
            id: rand::random::<u32>(),
            radius,
            position,
            mass: 5.0,
            velocity,
            acceleration: [0.0, 0.0],
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }

    pub fn change_position(&mut self, p: [f32; 2]) {
        self.position = p;
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    pub fn velocity(&self) -> [f32; 2] {
        self.velocity
    }

    pub fn change_velocity(&mut self, v: [f32; 2]) {
        self.velocity = v;
    }

    pub fn acceleration(&self) -> [f32; 2] {
        self.acceleration
    }

    pub fn force(&self) -> [f32; 2] {
        self.acceleration.map(|x| x * self.mass)
    }

    pub fn momentum(&self) -> [f32; 2] {
        self.velocity.map(|x| x * self.mass)
    }

    pub fn peer_collision(&self, other: &Person) -> bool {
        let dist = collision_physics::distance2(self.position, other.position);
        if dist < self.radius + other.radius {
            true
        } else {
            false
        }
    }
}

impl Eq for Person {}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        if self.id == other.id {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
