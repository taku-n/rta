// Rusty Technical Analysis

fn window(s: &[f64], period: usize) -> Vec<Vec<f64>> {
// (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))

    let mut v: Vec<Vec<f64>> = Vec::new();

    for i in 0..=(s.len() - period) {
        v.push(s.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        // copied and owned
    }

    v
}

fn window_over(s: &[f64], period: usize) -> Vec<Vec<f64>> {
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

fn window_over_zero(s: &[f64], period: usize) -> Vec<Vec<f64>> {
// (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))

    let mut t = vec![0.0; period - 1];
    t.extend(s);

    window(&t, period)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
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
}
