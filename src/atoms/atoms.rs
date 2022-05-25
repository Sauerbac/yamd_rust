use na::Matrix3xX;

#[derive(Debug, PartialEq)]
pub struct Atoms {
    // pub positions: Array<f64, Dim<[usize; 2]>>,
    pub positions: Matrix3xX<f64>,
    pub velocities: Matrix3xX<f64>,
    pub forces: Matrix3xX<f64>,
    pub mass: f64,
}

impl Atoms {
    pub fn new(coords: Matrix3xX<f64>, atom_mass: f64) -> Atoms {
        Atoms {
            velocities: Matrix3xX::zeros(coords.ncols()),
            forces: Matrix3xX::zeros(coords.ncols()),
            positions: coords,
            mass: atom_mass,
        }
    }
    pub fn num_atoms(&self) -> usize {
        self.positions.ncols()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ndarray::array;
//     #[test]
//     fn test_ref_and_no_ref() {
//         let coords1 = array![[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]];
//         let coords2 = array![[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]];
//         let mut atoms1 = Atoms::new(coords1, 1.0);
//         let mut atoms2 = Atoms::new(coords2, 1.0);
//         assert_eq!(atoms1.verlet1(0.1), atoms2.verlet_ref1(0.1))
//     }
// }
