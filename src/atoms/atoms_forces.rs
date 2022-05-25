use crate::atoms::atoms::Atoms;
use na::MatrixSlice3x1;

fn euclidean_distance_3d(p: &MatrixSlice3x1<f64>, q: &MatrixSlice3x1<f64>) -> f64 {
    let diff = p - q;
    f64::sqrt(diff.map(|diff| diff.powi(2)).sum())
}

// fn lj_direct_summation(atoms: &Atoms, epsilon: f64, sigma: f64) {
//     for atom in atoms.
// }

impl Atoms {
    // pub fn forces(mut self, epsilon: f64, sigma: f64) -> Atoms {
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::Matrix3x2;
    #[test]
    fn test_euclidean() {
        let p1 = Matrix3x2::<f64>::zeros();
        let v1 = p1.column(0);

        let p2 = Matrix3x2::from_element(1.0);
        let v2 = p2.column(0);

        let res = euclidean_distance_3d(&v1, &v2);
        assert_eq!(f64::sqrt(3.0), res)
    }
}
