use crate::atoms::atoms::Atoms;
use ndarray::prelude::*;

fn euclidean_distance_3d(p: &ArrayView2<f64>, q: &ArrayView2<f64>) -> f64 {
    let diff = p - q;
    f64::sqrt(diff.mapv(|diff| diff.powi(2)).sum())
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
    use ndarray::ArrayView;
    #[test]
    fn test_euclidean() {
        let s1 = [0.0, 0.0, 0.0];
        let p1 = ArrayView::from_shape((3, 1), &s1).unwrap();

        let s2 = [1.0, 1.0, 1.0];
        let p2 = ArrayView::from_shape((3, 1), &s2).unwrap();

        let res = euclidean_distance_3d(&p1, &p2);
        println!("{:?}", res);
        assert_eq!(f64::sqrt(3.0), res)
    }
}
