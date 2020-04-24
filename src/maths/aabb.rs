use cgmath::Vector3;

/// AABC is an Axis-Aligned Bounding Cube. it is used to check
/// for collisions in collision detection.
#[derive(Debug)]
pub struct AABC {
    pub center_pos: Vector3<f32>,
    pub half_size: Vector3<f32>,
}

impl AABC {
    // Returns true if the AABC is touching the
    // other AABC
    pub fn collides_with(&self, other: &AABC) -> bool {
        // AABBs are in collision with each other if and only
        // if, on all axes, the distance between the center of
        // the AABBs is less than the sum of half of the size of
        // either AABB.

        // Distance between centers
        let x_center_delta = (other.center_pos.x - self.center_pos.x).abs();
        let y_center_delta = (other.center_pos.y - self.center_pos.y).abs();
        let z_center_delta = (other.center_pos.z - self.center_pos.z).abs();

        // Sum of half sizes
        let x_half_size_sum = other.half_size.x + self.half_size.x;
        let y_half_size_sum = other.half_size.y + self.half_size.y;
        let z_half_size_sum = other.half_size.z + self.half_size.z;

        // On which axes do the AABBs collide?
        let x_collision = x_center_delta < x_half_size_sum;
        let y_collision = y_center_delta < y_half_size_sum;
        let z_collision = z_center_delta < z_half_size_sum;

        x_collision && y_collision && z_collision
    }
}
