extern crate nalgebra as na;
mod atoms;
use atoms::Atoms;
use na::{Matrix3xX, Vector3};
use std::thread::sleep;
use std::time::Instant;

fn main() {
    // // let coords1 = array![[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]];
    let coords1 = Matrix3xX::zeros(1000);
    let mut atoms1 = Atoms::new(coords1, 1.0);
    println!("{}", atoms1.num_atoms());
    println!("{:p}", &atoms1);
    let now = Instant::now();
    for i in 0..10000 {
        atoms1.verlet1_backup(0.1);
        atoms1.verlet2_backup(0.1)
    }
    println!("{}", now.elapsed().as_micros());

    println!("{:p}", &atoms1);
    let now2 = Instant::now();
    for i in 0..10000 {
        atoms1 = atoms1.verlet1(0.1);
        atoms1 = atoms1.verlet2(0.1);
    }
    println!("{:p}", &atoms1);

    println!("{}", now2.elapsed().as_micros());

    let p1 = Vector3::new(0.0, 0.0, 0.0).shape();
    println!("{:?}", p1);
}
