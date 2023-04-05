use ndarray::{arr1, Array1};

pub fn distance2(point1: [f32; 2], point2: [f32; 2]) -> f32 {
    let x = point1[0] - point2[0];
    let y = point1[1] - point2[1];
    let combo = x.powi(2) + y.powi(2);
    combo.sqrt()
}

fn dot2(l1: [f32; 2], l2: [f32; 2]) -> f32 {
    l1[0] * l2[0] + l1[1] * l2[1]
}

fn to_arr2(x: Array1<f32>) -> [f32; 2] {
    let mut arr = [0.0, 0.0];
    for i in 0..2 {
        arr[i] = x[i]
    }
    arr
}

pub fn colission_update2(
    l1: [f32; 2],
    m1: f32,
    x1: [f32; 2],
    l2: [f32; 2],
    m2: f32,
    x2: [f32; 2],
) -> [[f32; 2]; 2] {
    let v1 = arr1(&l1);
    let v2 = arr1(&l2);

    let x1 = arr1(&x1);
    let x2 = arr1(&x2);

    let x_diff1 = &x1 - &x2;
    let x_diff2 = &x2 - &x1;

    let m_total = m1 + m2;
    let d1 = dot2(to_arr2(&v1 - &v2), to_arr2(x_diff1.clone()));
    let d2 = dot2(to_arr2(&v2 - &v1), to_arr2(x_diff2.clone()));
    let length = |a: &Array1<f32>| a.map(|x| x * x).sum(); //.sqrt;

    let v2p = v1 - (2.001 * m2 / m_total) * (d1 / length(&x_diff1)) * (x_diff1);
    let v1p = v2 - (2.001 * m1 / m_total) * (d2 / length(&x_diff2)) * (x_diff2);
    [to_arr2(v1p), to_arr2(v2p)]
}

#[cfg(test)]
mod tests {

    use super::*;

    // Private unit tests
    #[test]
    fn dot2_product() {
        assert_eq!((dot2([5.5, 4.3], [1.1, 234.0]) * 100.0).floor(), 101225.0);
    }

    #[test]
    fn dot2_fail() {
        assert_ne!((dot2([5.5, 4.3], [-1.1, 234.0]) * 100.0).floor(), 101225.0);
    }

    //public unit tests
    #[test]
    fn distance() {
        assert_eq!(distance2([10.0, 10.0], [0.0, 0.0]), (200.0 as f32).sqrt());
    }

    #[test]
    fn distance_fail() {
        assert_ne!(distance2([11.0, 10.0], [0.0, 0.0]), (200.0 as f32).sqrt());
    }

    #[test]
    fn collision() {
        let p1 = ([5.0, 2.3], 20.0, [5.0, 2.3]);
        let p2 = ([-3.0, -1.0], 20.0, [2.0, 3.0]);
        assert_eq!(
            colission_update2(p1.0, p1.1, p1.2, p2.0, p2.1, p2.2),
            [[3.8601198, -2.6006947], [-1.8601198, 3.9006948]]
        );
    }
}
