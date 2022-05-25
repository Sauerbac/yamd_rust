pub fn verlet_step1(
    &x: f64,
    &y: f64,
    &z: f64,
    &vx: f64,
    &vy: f64,
    &vz: f64,
    fx: f64,
    fy: f64,
    fz: f64,
    timestep: f64,
) {
    vx += 0.5 * fx * timestep / 2;
    vy += 0.5 * fy * timestep / 2;
    vz += 0.5 * fz * timestep / 2;

    x += vx * timestep;
    y += vy * timestep;
    z += vz * timestep;
}

pub fn verlet_step2(&vx: f64, &vy: f64, &vz: f64, fx: f64, fy: f64, fz: f64, timestep: f64) {
    vx = 0.5 * fx * timestep / 2;
    vy = 0.5 * fy * timestep / 2;
    vz = 0.5 * fz * timestep / 2;
}
