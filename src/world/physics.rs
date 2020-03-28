enum Axis {
    X,
    Y,
    Z,
}

// fixes a collision given a breach value
fn fix(p: &mut PhysicalObject, breach: Vector3) {
    let smallest_non_zero_breach_axis = Axis::x;
    let smallest_non_zero_breach_value = breach.x;

    if (breach.y < smallest_non_zero_breach_value && breach.y != 0) || smallest_non_zero_breach_value == 0 {
        smallest_non_zero_breach_axis = Axis::y;
        smallest_non_zero_breach_value = breach.y;
    } 

    if (breach.z < smallest_non_zero_breach_value && breach.z != 0) || smallest_non_zero_breach_value == 0  {
        smallest_non_zero_breach_axis = Axis::z;
        smallest_non_zero_breach_value = breach.z;
    }

    if smallest_non_zero_breach_value == 0 {
        return
    }

    match smallest_non_zero_breach_axis {
        Axis::x => {
            let y_per_x = p.velocity.y / p.velocity.x;
            let z_per_x = p.velocity.z / p.velocity.x;

            p.add_position(Vector3{x: -breach.x});
            p.add_position(Vector3{y: -breach.x * y_per_x});
            p.add_position(Vector3{z: -breach.x * z_per_x});
        },
        Axis::y => {
            let x_per_y = p.velocity.x / p.velocity.y;
            let z_per_y = p.velocity.z / p.velocity.y;

            p.add_position(Vector3{x: -breach.y * x_per_y});
            p.add_position(Vector3{y: -breach.y});
            p.add_position(Vector3{z: -breach.y * z_per_y});
        }
        Axis::z => {
            let x_per_z = p.velocity.x / p.velocity.z;
            let y_per_z = p.velocity.y / p.velocity.z;

            p.add_position(Vector3{x: -breach.z * x_per_z});
            p.add_position(Vector3{y: -breach.z * y_per_z});
            p.add_position(Vector3{z: -breach.z});
        }
    }
}

fn apply_momentum(p1: &PhysicalObject, p2 &PhysicalObject) {
    // velocity
    let vi1 = p1.velocity;
    let vi2 = p2.velocity;

    // mass
    let m1 = p1.mass;
    let m2 = p2.mass;

    // momentum = velocity * mass
    let pi1 = Vector3 {
        x: vi1.x * m1, 
        y: vi1.y * m1, 
        z: vi1.z * m1,
    };
    let pi2 = Vector3{
        x: vi2.x * m2, 
        y: vi2.y * m2, 
        z: vi2.z * m2,
    };

    // kinetic energy = momentum * vel / 2 (who comes up
    // with this crap?)
    let ei1 = Vector3{
        x: pi1.x * vi1.x / 2, 
        y: pi1.y * vi1.y / 2, 
        z: pi1.z * vi1.z / 2,
    };
    let ei2 = Vector3{
        x: pi2.x * vi2.x / 2, 
        y: pi2.y * vi2.y / 2, 
        z: pi2.z * vi2.z / 2,
    };

    // so...
    // sum of final momentums = sum of initial momentums
    // and
    // sum of final kinetic energies = sum of initial kinetic energies

    // sum of momentum
    let sp = Vector3{
        x: pi1.x + pi2.x, 
        y: pi1.y + pi2.y, 
        z: pi1.z + pi2.z,
    };

    // sum of kinetic energy
    let s_ke = Vector3{
        x: ei1.x + ei2.x, 
        y: ei1.y + ei2.y, 
        z: ei1.z + ei2.z,
    };

    // final velocity calculation
    p1.velocity.x, p2.velocity.x = get_final_velocities(m1, m2, sp.x, s_ke.x);
    p1.velocity.y, p2.velocity.y = get_final_velocities(m1, m2, sp.y, s_ke.y);
    p1.velocity.z, p2.velocity.z = get_final_velocities(m1, m2, sp.z, s_ke.z);
}

// calculate final velocities along one axis
fn get_final_velocities(m1: f32, m2: f32, sp: f32, sKE: f32) -> (f32, f32) {
    let sqrt = (m2 * (sp*sp*(2*m2-m1) - 2*sKE*m1*(m1-m2))).sqrt();
    let vf2 = (m2*sp + sqrt) / (m2 * (m1 + m2));
    let vf1 = (sp - m2*vf2) / m1;

    (vf1, vf2)
}
