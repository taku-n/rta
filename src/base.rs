pub fn move_v2s(s_dst: &mut [f64], v_src: Vec<f64>) {
    for (i, x) in v_src.into_iter().enumerate() {
        s_dst[i] = x;
    }
}

pub fn window(s: &[f64], period: usize) -> Vec<Vec<f64>> {
// (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))

    let mut v: Vec<Vec<f64>> = Vec::new();

    for i in 0..=(s.len() - period) {
        v.push(s.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        // copied and owned
    }

    v
}

pub fn window_over(s: &[f64], period: usize) -> Vec<Vec<f64>> {
// (1 2 3 4 5) => ((1) (1 2) (1 2 3) (2 3 4) (3 4 5))

    let mut v: Vec<Vec<f64>> = Vec::new();

    for i in 1..period {
        v.push(s.to_vec().into_iter().take(i).collect::<Vec<_>>());
        // copied and owned
    }
    // create shorter vectors than period

    for i in 0..=(s.len() - period) {
        v.push(s.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        // copied and owned
    }

    v
}

pub fn window_over_zero(s: &[f64], period: usize) -> Vec<Vec<f64>> {
// (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))

    let mut t = vec![0.0; period - 1];
    t.extend(s);

    window(&t, period)
}

pub fn average(s: &[f64]) -> f64 {
    s.iter().fold(0.0, |acc, x| acc + x / s.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    fn move_v2s_test() {
        let v_src = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let mut array = [0.0; 8];
        let mut s_dst = &mut array[..];

        move_v2s(&mut s_dst, v_src);

        assert_eq!(s_dst, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn window_test() {
        assert!(window(&[1.0, 2.0, 3.0, 4.0, 5.0], 3)
                == [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }

    #[test]
    fn window_over_test() {
        assert!(window_over(&[1.0, 2.0, 3.0, 4.0, 5.0], 3)
                == [vec![1.0], vec![1.0, 2.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
    }

    #[test]
    fn window_over_zero_test() {
        assert!(window_over_zero(&[1.0, 2.0, 3.0, 4.0, 5.0], 3)
                == [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }

    #[test]
    fn average_test() {
        assert!(average(&[1.0, 2.0, 3.0, 4.0]) == 2.5);
    }

}