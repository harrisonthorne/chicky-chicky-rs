use maths;

// Gravity is the gravity acceleration constant (m/s/s)
const Gravity = -9.81

// PhysicalObject is an object with physics, position,
// velocity, and mass
struct PhysicalObject {
	frozen bool // if true, the PhysicalObject will not move

	velocity     maths.Vec3
	acceleration maths.Vec3
	mass         f32 // In kilograms.

	onGround    bool
	pushingWall bool
	atCeiling   bool

	wasOnGround    bool
	wasPushingWall bool
	wasAtCeiling   bool

	OnGroundhit  fn()
	OnPush       fn()
	OnCeilinghit fn()

	hitbox &maths.AABC // hitbox for collision calculation, but not kill calculation
}

// Position returns the PhysicalObject's position
fn Position(&self) maths.Vec3 {
	return p.hitbox.CenterPos
}

fn AddPosition(&self, v2 maths.Vec3) {
	p.hitbox.CenterPos.Add(v2)
}

// SetPosition modifies the position of the PhysicalObject.
fn SetPosition(&self, pos maths.Vec3) {
	p.hitbox.CenterPos = pos
}

// Physics calculates physics on the PhysicalObject p.
fn Physics(&self, delta f32) {
	// no physics if p is frozen
	if p.frozen {
		return
	}

	// gravity only applies if the PhysicalObject is not on
	// ground
	if !p.onGround {
		p.acceleration.Y += Gravity
	}

	p.acceleration.Scale(delta) // converts to velocity
	p.velocity.Add(p.acceleration)

	p.velocity.Scale(delta) // converts to displacement
	p.hitbox.CenterPos.Add(p.velocity)

	// reset acceleration
	p.acceleration.X = 0
	p.acceleration.Y = 0
	p.acceleration.Z = 0
}

// ApplyForce applies a force, in newtons, to the
// PhysicalObject. This is the only way to move a
// PhysicalObject in the game; velocity and acceleration are
// not publicly accessible.
fn ApplyForce(&self, newtons maths.Vec3) {
	p.acceleration.X += newtons.X / p.mass
	p.acceleration.Y += newtons.Y / p.mass
	p.acceleration.Z += newtons.Z / p.mass
}

// StopMotion immediately stops the motion of the
// PhysicalObject. Velocity and acceleration are set to
// zero.
fn StopMotion(&self) {
	p.velocity.X = 0
	p.velocity.Y = 0
	p.velocity.Z = 0
	p.acceleration.X = 0
	p.acceleration.Y = 0
	p.acceleration.Z = 0
}

// CollidesWith returns whether or not the Collider
// collides with another Collider
fn CollidesWith(&self, other &PhysicalObject) bool {
	return p.hitbox.CollidesWith(other.hitbox)
}

// FixCollision fixes a collision between two
// PhysicalObjects. If both objects are actively subject to
// forces, momentum will take effect on both PhysicalObjects
// and force will be applied to both of  them
fn FixCollision(&self, other &PhysicalObject) {
	if p.frozen || !p.CollidesWith(other) {
		return
	}

	// fix collisions
	switch {
	case !other.frozen:
		// fix both
		firstBreach = calculateBreach(p, other)
		secondBreach = calculateBreach(other, p)

		firstBreach.Scale(0.5)
		secondBreach.Scale(0.5)

		fix(p, firstBreach)
		fix(other, secondBreach)

		applyMomentum(p, other)
	case other.frozen:
		breach = calculateBreach(p, other)

		// fix p
		fix(p, breach)

		// now, we need to determine on which side of
		// `other` p is on. if it's on the top or bottom,
		// velocity stops on the y axis. if left or right, x
		// axis. first, re-calculate the breach now that
		// we've fixed the objects:
		breach = calculateBreach(p, other)

		// smallest breach determines which side the object is on
		minBreach = math.Min(breach.X), math.Min(f64(breach.Y), f64(breach.Z)) as f64 as f32
		switch minBreach {
		case breach.X:
			p.velocity.X = 0
		case breach.Y:
			p.velocity.Y = 0
		case breach.Z:
			p.velocity.Z = 0
		}
	}
}

fn calculateBreach(moving, static &PhysicalObject) (breach maths.Vec3) {
	// breach really depends on which direction the moving
	// PhysicalObject is travelling

	// calculate X
	switch {
	case moving.velocity.X > 0:
		breach.X = moving.hitbox.CenterPos.X + moving.hitbox.HalfSize.X - (static.hitbox.CenterPos.X - static.hitbox.HalfSize.X)
	case moving.velocity.X < 0:
		breach.X = moving.hitbox.CenterPos.X - moving.hitbox.HalfSize.X - (static.hitbox.CenterPos.X + static.hitbox.HalfSize.X)
	}

	// calculate Y
	switch {
	case moving.velocity.Y > 0:
		breach.Y = moving.hitbox.CenterPos.Y + moving.hitbox.HalfSize.Y - (static.hitbox.CenterPos.Y - static.hitbox.HalfSize.Y)
	case moving.velocity.Y < 0:
		breach.Y = moving.hitbox.CenterPos.Y - moving.hitbox.HalfSize.Y - (static.hitbox.CenterPos.Y + static.hitbox.HalfSize.Y)
	}

	// calculate Z
	switch {
	case moving.velocity.Z > 0:
		breach.Z = moving.hitbox.CenterPos.Z + moving.hitbox.HalfSize.Z - (static.hitbox.CenterPos.Z - static.hitbox.HalfSize.Z)
	case moving.velocity.Z < 0:
		breach.Z = moving.hitbox.CenterPos.Z - moving.hitbox.HalfSize.Z - (static.hitbox.CenterPos.Z + static.hitbox.HalfSize.Z)
	}
	return
}

// vim: foldmethod=syntax
