extern crate nalgebra as na;
mod atoms;
use atoms::{io, Atoms};
use na::{Matrix3x2, Matrix3xX, Vector3};
use std::thread::sleep;
use std::time::Instant;

fn main() {
    let a = Atoms::read(&String::from(
        "/home/sauerbach/HPC/yamd_rust/static/lj54.xyz",
    ));
    // io::p(&a.positions);
    // io::p(&a.mass);
    a.write("/home/sauerbach/HPC/yamd_rust/static/test.xyz")
}
