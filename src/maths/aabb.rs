/// AABC is an Axis-Aligned Bounding Cube. it is used to check
/// for collisions in collision detection.
struct AABC {
    center_pos: Vector3,
    half_size:  Vector3,
}

impl AABC {
    // Returns true if the AABC is touching the
    // other AABC
    fn collides_with(&self, other: &AABC) -> bool {
        // AABBs are in collision with each other if and only
        // if, on all axes, the distance between the center of
        // the AABBs is less than the sum of half of the size of
        // either AABB.

        // Distance between centers
        let x_center_delta = (other.center_pos.x - a.center_pos.x).abs();
        let y_center_delta = (other.center_pos.y - a.center_pos.y).abs();
        let z_center_delta = (other.center_pos.z - a.center_pos.z).abs();

        // Sum of half sizes
        let x_half_size_sum = other.half_size.x + a.half_size.x;
        let y_half_size_sum = other.half_size.y + a.half_size.y;
        let z_half_size_sum = other.half_size.z + a.half_size.z;

        // On which axes do the AABBs collide?
        let x_collision = x_center_delta < x_half_size_sum;
        let y_collision = y_center_delta < y_half_size_sum;
        let z_collision = z_center_delta < z_half_size_sum;

        x_collision && y_collision && z_collision
    }
}
