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

    pub fn new_cubic(num_cells: usize, cell_side_len: f64) {
        let num_cells_one_dim = (num_cells as f64).powf(1.0 / 3.0).ceil();
        let x_positions: Vec<f64> = Vec::new();
        for i in 0..=num_cells {}
    }
    pub fn num_atoms(&self) -> usize {
        self.positions.ncols()
    }
}
