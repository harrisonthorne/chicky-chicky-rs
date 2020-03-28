use crate::maths;

/// The gravity acceleration constant (m/s/s)
const GRAVITY: f32 = -9.81;

/// PhysicalObject is an object with physics, position,
/// velocity, and mass
struct PhysicalObject {
    /// if true, the PhysicalObject will not move
    frozen: bool,

    velocity: Vector3,
    acceleration: Vector3,

    /// In kilograms.
    mass: f32,

    on_ground: bool,
    pushing_wall: bool,
    at_ceiling: bool,

    was_on_ground: bool,
    was_pushing_wall: bool,
    was_at_ceiling: bool,

    hitbox: maths::AABC, // hitbox for collision calculation, but not kill calculation
}

impl PhysicalObject {
    /// Returns the PhysicalObject's position
    fn position(&self) -> Vector3 {
        self.hitbox.center_pos;
    }

    fn add_position(&self, v2: Vector3) {
        self.hitbox.center_pos.Add(v2)
    }

    /// Modifies the position of the PhysicalObject.
    fn set_position(&self, pos: Vector3) {
        self.hitbox.center_pos = pos
    }

    /// Calculates physics on the PhysicalObject.
    fn physics(&self, delta: f32) {
        // no physics if p is frozen
        if self.frozen {
            return;
        }

        // gravity only applies if the PhysicalObject is not on
        // ground
        if !self.on_ground {
            self.acceleration.y += gravity;
        }

        self.acceleration.scale(delta); // converts to velocity
        self.velocity.add(self.acceleration);

        self.velocity.scale(delta); // converts to displacement
        self.hitbox.center_pos.add(self.velocity);

        // reset acceleration
        self.acceleration.x = 0;
        self.acceleration.y = 0;
        self.acceleration.z = 0;
    }

    /// Applies a force, in newtons, to the PhysicalObject. This is the only way to move a
    /// PhysicalObject in the game; velocity and acceleration are not publicly accessible.
    fn apply_force(&self, newtons: Vector3) {
        self.acceleration.x += newtons.x / self.mass;
        self.acceleration.y += newtons.y / self.mass;
        self.acceleration.z += newtons.z / self.mass;
    }

    /// Immediately stops the motion of the PhysicalObject. Velocity and acceleration are set to
    /// zero.
    fn stop_motion(&self) {
        self.velocity.x = 0;
        self.velocity.y = 0;
        self.velocity.z = 0;
        self.acceleration.x = 0;
        self.acceleration.y = 0;
        self.acceleration.z = 0;
    }

    /// Returns whether or not the PhysicalObjects collide.
    fn collides_with(&self, other: &PhysicalObject) -> bool {
        self.hitbox.collides_with(other.hitbox)
    }

    /// FixCollision fixes a collision between two PhysicalObjects. If both objects are actively
    /// subject to forces, momentum will take effect on both PhysicalObjects and force will be
    /// applied to both of  them
    fn fix_collision(&self, other: &PhysicalObject) {
        if self.frozen || !self.collides_with(other) {
            return;
        }

        // fix collisions
        if !other.frozen {
            // fix both
            first_breach = calculate_breach(p, other);
            second_breach = calculate_breach(other, p);

            first_breach.scale(0.5);
            second_breach.scale(0.5);

            fix(p, first_breach);
            fix(other, second_breach);

            apply_momentum(p, other);
        } else if other.frozen {
            breach = calculate_breach(p, other);

            // fix p
            fix(p, breach);

            // now, we need to determine on which side of
            // `other` p is on. if it's on the top or bottom,
            // velocity stops on the y axis. if left or right, x
            // axis. first, re-calculate the breach now that
            // we've fixed the objects:
            breach = calculate_breach(p, other);

            // smallest breach determines which side the object is on
            min_breach = breach.x.min(breach.y.min(breach.z));

            if breach.x == minBreach {
                self.velocity.x = 0;
            } else if breach.y == minBreach {
                self.velocity.y = 0;
            } else if breach.z == minBreach {
                self.velocity.z = 0;
            }
        }
    }
}

/// Returns a breach.
fn calculate_breach(moving: &PhysicalObject, still: &PhysicalObject) -> Vector3 {
    // breach really depends on which direction the moving
    // PhysicalObject is travelling

    // calculate x
    if moving.velocity.x > 0 {
        breach.x = moving.hitbox.center_pos.x + moving.hitbox.half_size.x
            - (still.hitbox.center_pos.x - still.hitbox.half_size.x);
    } else if moving.velocity.x < 0 {
        breach.x = moving.hitbox.center_pos.x
            - moving.hitbox.half_size.x
            - (still.hitbox.center_pos.x + still.hitbox.half_size.x);
    }

    // calculate y
    if moving.velocity.y > 0 {
        breach.y = moving.hitbox.center_pos.y + moving.hitbox.half_size.y
            - (still.hitbox.center_pos.y - still.hitbox.half_size.y);
    } else if moving.velocity.y < 0 {
        breach.y = moving.hitbox.center_pos.y
            - moving.hitbox.half_size.y
            - (still.hitbox.center_pos.y + still.hitbox.half_size.y);
    }

    // calculate z
    if moving.velocity.z > 0 {
        breach.z = moving.hitbox.center_pos.z + moving.hitbox.half_size.z
            - (still.hitbox.center_pos.z - still.hitbox.half_size.z);
    } else if moving.velocity.z < 0 {
        breach.z = moving.hitbox.center_pos.z
            - moving.hitbox.half_size.z
            - (still.hitbox.center_pos.z + still.hitbox.half_size.z);
    }

    breach
}

// vim: foldmethod=syntax
