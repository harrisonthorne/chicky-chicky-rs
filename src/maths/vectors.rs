

// Vector3 is a vector with two components. This is intended
// for use with velocity, position, and acceleration.
struct Vector3 {
    X f32
    Y f32
}

// Add adds the components of v2 to the matching
// components of Vector3 v1
fn Add(&self, v2 Vector3) {
    v.X += v2.X
    v.Y += v2.Y
}

// Scale scales the vector
fn Scale(&self, scalar f32) {
    v.X *= scalar
    v.Y *= scalar
}

// Vector3 is a vector with three components. This is intended
// for use with three-dimensional velocity, position, and
// acceleration.
struct Vector3 {
    X f32
    Y f32
    Z f32
}

// Add adds the components of v2 to the matching
// components of Vector3 v1
fn Add(&self, v2 Vector3) {
    v.X += v2.X
    v.Y += v2.Y
    v.Z += v2.Z
}

// Scale scales the vector
fn Scale(&self, scalar f32) {
    v.X *= scalar
    v.Y *= scalar
    v.Z *= scalar
}
