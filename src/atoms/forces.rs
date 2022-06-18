use crate::atoms::atoms::Atoms;
use na::{MatrixSlice3x1, Vector3};

fn lennard_jones_forces(
    pos1: &MatrixSlice3x1<f64>,
    pos2: &MatrixSlice3x1<f64>,
    epsilon: f64,
    sigma: f64,
) -> Vector3<f64> {
    let distance = pos1.metric_distance(&pos2);
    let pauli = 12.0 * sigma.powi(12) / distance.powi(13);
    let london = 6.0 * sigma.powi(6) / distance.powi(7);
    4.0 * epsilon * (pauli - london) * (pos1 - pos2).normalize()
}

impl Atoms {
    pub fn calc_forces(mut self, epsilon: f64, sigma: f64) -> Atoms {
        for (i, v1) in self.positions.column_iter().enumerate() {
            let mut force_sum = Vector3::zeros();
            for (j, v2) in self.positions.column_iter().enumerate() {
                if i != j {
                    let force = lennard_jones_forces(&v1, &v2, epsilon, sigma);
                    force_sum = force_sum + force;
                }
            }
            self.forces.set_column(i, &force_sum);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::{Matrix3xX, Vector3};
    #[test]
    fn test_forces_two_atoms() {
        let mut two_atoms = Atoms::new(2);

        two_atoms
            .positions
            .set_column(0, &Vector3::from_element(0.0));
        two_atoms
            .positions
            .set_column(1, &Vector3::new(1.0, 1.0, 1.0));
        two_atoms = two_atoms.calc_forces(1.0, 1.0);

        let mut solution = Matrix3xX::<f64>::zeros(2);
        solution.set_column(0, &Vector3::from_element(200.0 / 729.0));
        solution.set_column(1, &Vector3::from_element(-200.0 / 729.0));

        println!(
            "computed forces: {} analytical solution {}",
            &two_atoms.forces, &solution
        );

        let difference_of_sums = (&two_atoms.forces - solution).sum();
        println!("{}", &difference_of_sums);
        assert_eq!(0.0, difference_of_sums);
    }

    #[test]
    fn test_three_atoms_symetrically() {
        let mut three_atoms = Atoms::new(3);

        three_atoms
            .positions
            .set_column(0, &Vector3::new(-1.0, -1.0, -1.0));
        three_atoms
            .positions
            .set_column(1, &Vector3::from_element(0.0));
        three_atoms
            .positions
            .set_column(2, &Vector3::new(1.0, 1.0, 1.0));
        three_atoms = three_atoms.calc_forces(1.0, 1.0);

        println!("{}", &three_atoms.forces);
        assert_eq!(0.0, three_atoms.forces.column(1).sum())
    }
}
