// AABC is an Axis-Aligned Bounding Cube. it is used to check
// for collisions in collision detection.
struct AABC {
	CenterPos Vec3
	HalfSize  Vec3
}

// CollidesWith returns true if the AABC is touching the
// other AABB
fn CollidesWith(&self, other &AABC) bool {
    // AABBs are in collision with each other if and only
    // if, on all axes, the distance between the center of
    // the AABBs is less than the sum of half of the size of
    // either AABB.

    // Distance between centers
    xCenterDelta = math.Abs(other.CenterPos.X - a.CenterPos.X) as f64
    yCenterDelta = math.Abs(other.CenterPos.Y - a.CenterPos.Y) as f64
    zCenterDelta = math.Abs(other.CenterPos.Z - a.CenterPos.Z) as f64

    // Sum of half sizes
    xHalfSizeSum = other.HalfSize.X + a.HalfSize.X
    yHalfSizeSum = other.HalfSize.Y + a.HalfSize.Y
    zHalfSizeSum = other.HalfSize.Z + a.HalfSize.Z

    // On which axes do the AABBs collide?
    xCollision = xCenterDelta as f32 < xHalfSizeSum
    yCollision = yCenterDelta as f32 < yHalfSizeSum
    zCollision = zCenterDelta as f32 < zHalfSizeSum
    
    return xCollision && yCollision && zCollision
}

