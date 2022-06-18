use na::Matrix3xX;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Atoms {
    // pub positions: Array<f64, Dim<[usize; 2]>>,
    pub positions: Matrix3xX<f64>,
    pub velocities: Matrix3xX<f64>,
    pub forces: Matrix3xX<f64>,
    pub mass: f64,
}

impl Atoms {
    pub fn new(num: usize) -> Atoms {
        Atoms {
            velocities: Matrix3xX::zeros(num),
            forces: Matrix3xX::zeros(num),
            positions: Matrix3xX::zeros(num),
            mass: 1.0,
        }
    }

    pub fn new_from_coords(coords: Matrix3xX<f64>, atom_mass: f64) -> Atoms {
        Atoms {
            velocities: Matrix3xX::zeros(coords.ncols()),
            forces: Matrix3xX::zeros(coords.ncols()),
            positions: coords,
            mass: atom_mass,
        }
    }

    pub fn new_cubic(
        num_atoms: usize,
        cell_side_len: f64,
        atom_mass: f64,
        offset_factor: Option<f64>,
    ) -> Atoms {
        let num_atoms_one_dim = (num_atoms as f64).powf(1.0 / 3.0).ceil() as usize;
        let mut positions: Vec<f64> = Vec::new();
        let mut rng = rand::thread_rng();

        if let Some(offset) = offset_factor {
            for i in 0..num_atoms_one_dim {
                for j in 0..num_atoms_one_dim {
                    for k in 0..num_atoms_one_dim {
                        positions.push(i as f64 + rng.gen::<f64>() * offset);
                        positions.push(j as f64 + rng.gen::<f64>() * offset);
                        positions.push(k as f64 + rng.gen::<f64>() * offset);
                    }
                }
            }
        } else {
            for i in 0..num_atoms_one_dim {
                for j in 0..num_atoms_one_dim {
                    for k in 0..num_atoms_one_dim {
                        positions.push(i as f64);
                        positions.push(j as f64);
                        positions.push(k as f64);
                    }
                }
            }
        }

        let mut lattice_pos = Matrix3xX::from_vec(positions);
        let num_cols = lattice_pos.ncols();
        println!("{:?}", num_cols);
        for i in 0..(num_cols - num_atoms) {
            let rand_index = rng.gen_range(0..(num_cols - i));
            lattice_pos = lattice_pos.remove_column(rand_index);
        }
        Atoms {
            velocities: Matrix3xX::zeros(lattice_pos.ncols()),
            forces: Matrix3xX::zeros(lattice_pos.ncols()),
            positions: lattice_pos,
            mass: atom_mass,
        }
    }

    pub fn num_atoms(&self) -> usize {
        self.positions.ncols()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atoms::{io, Atoms};

    #[test]
    fn see_cubic() {
        let mat = Atoms::new_cubic(9, 1.0, 1.0, None);
        io::p(mat.positions);
    }
}
