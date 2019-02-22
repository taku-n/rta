// Rusty Technical Analysis

mod base;
mod dbg;

use std::slice;

#[no_mangle]
pub extern "C" fn sma_c(p_dst: *mut f64, p_src: *const f64, n: usize,
        period: usize, begin: usize) {

    let mut dst = unsafe {slice::from_raw_parts_mut(p_dst, n)};
    let src = unsafe {slice::from_raw_parts(p_src, n)};

    let sma = sma(src, period, begin);

    base::move_v2s(dst, sma, begin);
}

fn sma(s: &[f64], period: usize, begin: usize) -> Vec<f64> {
    if begin < period - 1 {
        base::window_over_zero(s, period).into_iter().skip(begin)
                .map(|x| base::average(&x)).collect::<Vec<_>>()
    } else {
        let data = &s[(1 + begin - period)..];
        // This calculation order avoids "attempt to subtract with overflow".
        // If you calculate (begin - period + 1), you get an error of that.
        // The value can not be a negative value if only temporarily.

        base::window(data, period).into_iter()
                .map(|x| base::average(&x)).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sma_test() {
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let s = &array[..];

        assert_eq!(sma(s, 3, 0), [0.3333333333333333, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(sma(s, 3, 2), [2.0, 3.0, 4.0]);
    }
}
