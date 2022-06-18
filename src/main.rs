extern crate nalgebra as na;
mod atoms;
use atoms::{io, Atoms};
use na::Vector3;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let mut system = Atoms::read(&String::from(
        "/home/sauerbach/HPC/yamd_rust/static/lj54_novel.xyz",
    ));
    // let mut system = Atoms::gen_num_atoms(3);
    // system.positions.set_column(1, &Vector3::new(1.0, 0.0, 0.0));
    // system.positions.set_column(2, &Vector3::new(0.0, 1.0, 0.0));
    // system.mass = 1.008;

    let path = "/home/sauerbach/HPC/yamd_rust/static/test_novel.xyz";
    let mut f = BufWriter::new(File::create(path).expect("Unable to create file"));

    let timestep = 0.01;
    let epsilon = 0.5;
    let sigma = 0.9;
    for i in 0..10000 {
        // io::p(system.calc_kinetic_energy());
        // io::p(&system.velocities.row_sum() / 54.0);
        system = system.verlet1(timestep);
        system = system.calc_forces(epsilon, sigma);
        system = system.verlet2(timestep);
        // system = system.berendsen(temperature, timestep, relaxation_time);
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

    // let path = "/home/sauerbach/HPC/yamd_rust/static/test.xyz";
    // let mut f = BufWriter::new(File::create(path).expect("Unable to create file"));

    // let mut three_atoms = Atoms::gen_num_atoms(3);

    // three_atoms
    //     .positions
    //     .set_column(0, &Vector3::new(-1.0, -1.0, -1.0));
    // three_atoms
    //     .positions
    //     .set_column(1, &Vector3::from_element(0.0));
    // three_atoms
    //     .positions
    //     .set_column(2, &Vector3::new(1.0, 1.0, 1.0));
    // three_atoms = three_atoms.calc_forces(1.0, 1.0);

    // let timestep = 0.01;
    // for _ in 0..1000 {
    //     // io::p(system.calc_potential_energy(1.0, 1.0));
    //     three_atoms = three_atoms.verlet1(timestep);
    //     three_atoms = three_atoms.calc_forces(1.0, 1.0);
    //     io::p(&three_atoms.velocities);
    //     three_atoms = three_atoms.verlet2(timestep);
    //     io::p(&three_atoms.velocities);
    //     io::append_xyz(&three_atoms, &mut f)
    // }

    // system.write("/home/sauerbach/HPC/yamd_rust/static/test.xyz")
}
