extern crate nalgebra as na;
mod atoms;
use atoms::{io, Atoms};
use na::Vector3;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    // let mut system = Atoms::read(&String::from(
    //     "/home/sauerbach/HPC/yamd_rust/static/lj54_novel.xyz",
    let mut system = Atoms::new_cubic(150, 1.0, 1.0, Some(0.5));
    system.mass = 1.008;

    let path = "/home/sauerbach/HPC/yamd_rust/static/test.xyz";
    let mut f = BufWriter::new(File::create(path).expect("Unable to create file"));

    let timestep = 0.01;
    let epsilon = 0.2;
    let sigma = 1.0;
    let mut goal_temp = 1000.0;
    let relaxation_time = 10.0;
    for i in 0..10000 {
        system = system.verlet1(timestep);
        system = system.calc_forces(epsilon, sigma);
        system = system.verlet2(timestep);
        system = system.berendsen(goal_temp, timestep, relaxation_time);
        // goal_temp += 1.0;
        if i % 10 == 0 {
            io::append_xyz(&system, &mut f);
            // io::p(i);
            io::p(format!(
                "Pot: {}, Kin: {}, Gesamt: {}, Temp: {}",
                system.potential_energy(epsilon, sigma),
                system.kinetic_energy(),
                system.total_energy(epsilon, sigma),
                system.temperature()
            ));
        }
    }
}
