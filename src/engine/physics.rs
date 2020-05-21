use crate::maths;
use cgmath::Vector3;

/// The gravity acceleration constant (m/s/s)
const GRAVITY: f32 = -9.81;

/// PhysicalObject is an object with physics, position,
/// velocity, and mass
#[derive(Debug)]
pub struct PhysicalObject {
    /// if true, the PhysicalObject will not move
    frozen: bool,

    velocity: Vector3<f32>,
    acceleration: Vector3<f32>,

    /// In kilograms
    mass: f32,

    on_ground: bool,
    pushing_wall: bool,
    at_ceiling: bool,

    was_on_ground: bool,
    was_pushing_wall: bool,
    was_at_ceiling: bool,

    hitbox: maths::AABB, // hitbox for collision calculation, but not kill calculation
}

impl PhysicalObject {
    pub fn new(mass: f32, physics_box: maths::AABB) -> Self {
        Self {
            frozen: false,

            velocity: Vector3::from((0.0, 0.0, 0.0)),
            acceleration: Vector3::from((0.0, 0.0, 0.0)),

            mass,

            on_ground: false,
            pushing_wall: false,
            at_ceiling: false,

            was_on_ground: false,
            was_pushing_wall: false,
            was_at_ceiling: false,

            hitbox: physics_box,
        }
    }

    /// Returns the PhysicalObject's position
    fn position(&self) -> Vector3<f32> {
        self.hitbox.center_pos
    }

    fn add_position(&mut self, v2: Vector3<f32>) {
        self.hitbox.center_pos += v2;
    }

    /// Modifies the position of the PhysicalObject.
    fn set_position(&mut self, pos: Vector3<f32>) {
        self.hitbox.center_pos = pos;
    }

    /// Calculates physics on the PhysicalObject.
    fn physics(&mut self, delta: f32) {
        // no physics if p is frozen
        if self.frozen {
            return;
        }

        // gravity only applies if the PhysicalObject is not on
        // ground
        if !self.on_ground {
            self.acceleration.y += GRAVITY;
        }

        self.acceleration *= delta; // converts to velocity
        self.velocity += self.acceleration;

        self.velocity *= delta; // converts to displacement
        self.hitbox.center_pos += self.velocity;

        // reset acceleration
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        self.acceleration.z = 0.0;
    }

    /// Applies a force, in newtons, to the PhysicalObject. This is the only way to move a
    /// PhysicalObject in the game; velocity and acceleration are not publicly accessible.
    pub fn apply_force(&mut self, newtons: Vector3<f32>) {
        self.acceleration.x += newtons.x / self.mass;
        self.acceleration.y += newtons.y / self.mass;
        self.acceleration.z += newtons.z / self.mass;
    }

    /// Immediately stops the motion of the PhysicalObject. Velocity and acceleration are set to
    /// zero.
    fn stop_motion(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.velocity.z = 0.0;
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        self.acceleration.z = 0.0;
    }

    /// Returns whether or not the PhysicalObjects collide.
    fn collides_with(&self, other: &PhysicalObject) -> bool {
        self.hitbox.collides_with(&other.hitbox)
    }

    /// FixCollision fixes a collision between two PhysicalObjects. If both objects are actively
    /// subject to forces, momentum will take effect on both PhysicalObjects and force will be
    /// applied to both of them
    fn fix_collision(&mut self, other: &mut PhysicalObject) {
        if self.frozen || !self.collides_with(other) {
            return;
        }

        // fix collisions
        if !other.frozen {
            // fix both
            let mut first_breach = calculate_breach(self, other);
            let mut second_breach = calculate_breach(other, self);

            first_breach *= 0.5;
            second_breach *= 0.5;

            fix(self, first_breach);
            fix(other, second_breach);

            apply_momentum(self, other);
        } else if other.frozen {
            let mut breach = calculate_breach(self, other);

            // fix p
            fix(self, breach);

            // now, we need to determine on which side of
            // `other` p is on. if it's on the top or bottom,
            // velocity stops on the y axis. if left or right, x
            // axis. first, re-calculate the breach now that
            // we've fixed the objects:
            breach = calculate_breach(self, other);

            // smallest breach determines which side the object is on
            let min_breach = breach.x.min(breach.y.min(breach.z));

            if (breach.x - min_breach).abs() < FLOAT_ERROR {
                self.velocity.x = 0.0;
            } else if (breach.y - min_breach).abs() < FLOAT_ERROR {
                self.velocity.y = 0.0;
            } else if (breach.z - min_breach).abs() < FLOAT_ERROR {
                self.velocity.z = 0.0;
            }
        }
    }
}

const FLOAT_ERROR: f32 = 0.0001;

/// Returns a breach.
fn calculate_breach(moving: &PhysicalObject, still: &PhysicalObject) -> Vector3<f32> {
    let mut breach = Vector3::new(0.0, 0.0, 0.0);

    // breach really depends on which direction the moving
    // PhysicalObject is travelling

    // calculate x
    if moving.velocity.x > 0.0 {
        breach.x = moving.hitbox.center_pos.x + moving.hitbox.half_size.x
            - (still.hitbox.center_pos.x - still.hitbox.half_size.x);
    } else if moving.velocity.x < 0.0 {
        breach.x = moving.hitbox.center_pos.x
            - moving.hitbox.half_size.x
            - (still.hitbox.center_pos.x + still.hitbox.half_size.x);
    }

    // calculate y
    if moving.velocity.y > 0.0 {
        breach.y = moving.hitbox.center_pos.y + moving.hitbox.half_size.y
            - (still.hitbox.center_pos.y - still.hitbox.half_size.y);
    } else if moving.velocity.y < 0.0 {
        breach.y = moving.hitbox.center_pos.y
            - moving.hitbox.half_size.y
            - (still.hitbox.center_pos.y + still.hitbox.half_size.y);
    }

    // calculate z
    if moving.velocity.z > 0.0 {
        breach.z = moving.hitbox.center_pos.z + moving.hitbox.half_size.z
            - (still.hitbox.center_pos.z - still.hitbox.half_size.z);
    } else if moving.velocity.z < 0.0 {
        breach.z = moving.hitbox.center_pos.z
            - moving.hitbox.half_size.z
            - (still.hitbox.center_pos.z + still.hitbox.half_size.z);
    }

    breach
}

// utils for fixing breaches, collisions, etc

enum Axis {
    X,
    Y,
    Z,
}

// fixes a collision given a breach value
pub fn fix(p: &mut PhysicalObject, breach: Vector3<f32>) {
    let mut smallest_non_zero_breach_axis = Axis::X;
    let mut smallest_non_zero_breach_value = breach.x;

    if (breach.y < smallest_non_zero_breach_value && breach.y != 0.0)
        || smallest_non_zero_breach_value == 0.0
    {
        smallest_non_zero_breach_axis = Axis::Y;
        smallest_non_zero_breach_value = breach.y;
    }

    if (breach.z < smallest_non_zero_breach_value && breach.z != 0.0)
        || smallest_non_zero_breach_value == 0.0
    {
        smallest_non_zero_breach_axis = Axis::Z;
        smallest_non_zero_breach_value = breach.z;
    }

    if smallest_non_zero_breach_value == 0.0 {
        return;
    }

    match smallest_non_zero_breach_axis {
        Axis::X => {
            let y_per_x = p.velocity.y / p.velocity.x;
            let z_per_x = p.velocity.z / p.velocity.x;

            p.add_position(Vector3::from((
                -breach.x,
                -breach.x * y_per_x,
                -breach.x * z_per_x,
            )));
        }
        Axis::Y => {
            let x_per_y = p.velocity.x / p.velocity.y;
            let z_per_y = p.velocity.z / p.velocity.y;

            p.add_position(Vector3::from((
                -breach.y * x_per_y,
                -breach.y,
                -breach.y * z_per_y,
            )));
        }
        Axis::Z => {
            let x_per_z = p.velocity.x / p.velocity.z;
            let y_per_z = p.velocity.y / p.velocity.z;

            p.add_position(Vector3::from((
                -breach.z * x_per_z,
                -breach.z * y_per_z,
                -breach.z,
            )));
        }
    }
}

pub fn apply_momentum(p1: &mut PhysicalObject, p2: &mut PhysicalObject) {
    // velocity
    let vi1 = p1.velocity;
    let vi2 = p2.velocity;

    // mass
    let m1 = p1.mass;
    let m2 = p2.mass;

    // momentum = velocity * mass
    let pi1 = Vector3::from((vi1.x * m1, vi1.y * m1, vi1.z * m1));
    let pi2 = Vector3::from((vi2.x * m2, vi2.y * m2, vi2.z * m2));

    // kinetic energy = momentum * vel / 2.0 (who comes up
    // with this crap?)
    let ei1 = Vector3::from((
        pi1.x * vi1.x / 2.0,
        pi1.y * vi1.y / 2.0,
        pi1.z * vi1.z / 2.0,
    ));
    let ei2 = Vector3::from((
        pi2.x * vi2.x / 2.0,
        pi2.y * vi2.y / 2.0,
        pi2.z * vi2.z / 2.0,
    ));

    // so...
    // sum of final momentums = sum of initial momentums
    // and
    // sum of final kinetic energies = sum of initial kinetic energies

    // sum of momentum
    let sp = Vector3::from((pi1.x + pi2.x, pi1.y + pi2.y, pi1.z + pi2.z));

    // sum of kinetic energy
    let s_ke = Vector3::from((ei1.x + ei2.x, ei1.y + ei2.y, ei1.z + ei2.z));

    // final velocity calculation
    let final_vx = get_final_velocities(m1, m2, sp.x, s_ke.x);
    let final_vy = get_final_velocities(m1, m2, sp.y, s_ke.y);
    let final_vz = get_final_velocities(m1, m2, sp.z, s_ke.z);

    p1.velocity.x = final_vx.0;
    p1.velocity.y = final_vy.0;
    p1.velocity.z = final_vz.0;

    p2.velocity.x = final_vx.1;
    p2.velocity.y = final_vy.1;
    p2.velocity.z = final_vz.1;
}

// calculate final velocities along one axis
fn get_final_velocities(m1: f32, m2: f32, sp: f32, s_ke: f32) -> (f32, f32) {
    let sqrt = (m2 * (sp * sp * (2.0 * m2 - m1) - 2.0 * s_ke * m1 * (m1 - m2))).sqrt();
    let vf2 = (m2 * sp + sqrt) / (m2 * (m1 + m2));
    let vf1 = (sp - m2 * vf2) / m1;

    (vf1, vf2)
}

// vim: foldmethod=syntax
