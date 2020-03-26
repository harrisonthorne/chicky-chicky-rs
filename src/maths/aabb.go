// AABC is an Axis-Aligned Bounding Cube. it is used to check
// for collisions in collision detection.
struct AABC {
	center_pos Vec3
	half_size  Vec3
}

// CollidesWith returns true if the AABC is touching the
// other AABB
fn CollidesWith(&self, other &AABC) -> bool {
    // AABBs are in collision with each other if and only
    // if, on all axes, the distance between the center of
    // the AABBs is less than the sum of half of the size of
    // either AABB.

    // Distance between centers
    xCenterDelta = math.Abs(other.center_pos.X - a.center_pos.X) as f64
    yCenterDelta = math.Abs(other.center_pos.Y - a.center_pos.Y) as f64
    zCenterDelta = math.Abs(other.center_pos.Z - a.center_pos.Z) as f64

    // Sum of half sizes
    xhalf_sizeSum = other.half_size.X + a.half_size.X
    yhalf_sizeSum = other.half_size.Y + a.half_size.Y
    zhalf_sizeSum = other.half_size.Z + a.half_size.Z

    // On which axes do the AABBs collide?
    xCollision = xCenterDelta as f32 < xhalf_sizeSum
    yCollision = yCenterDelta as f32 < yhalf_sizeSum
    zCollision = zCenterDelta as f32 < zhalf_sizeSum
    
    return xCollision && yCollision && zCollision
}

