// Rusty Technical Analysis

mod base;
mod dbg;

use std::mem;
use std::slice;

#[no_mangle]
pub extern "C" fn sma_c(
        p_dest: *mut f64, p_src: *const f64, n: usize, period: usize) {

    //dbg::str(&format!("p_src => {:?}", p_src));
    //dbg::str(&format!("p_src (ptr) => {:p}", p_src));

    let mut dest = unsafe {slice::from_raw_parts_mut(p_dest, n)};
    let src = unsafe {slice::from_raw_parts(p_src, n)};

    //dbg::s_f64(&src[..10]);

    base::move_v2s(dest, sma(src, period));
}

fn sma(s: &[f64], period: usize) -> Vec<f64> {
    base::window_over_zero(s, period).into_iter()
            .map(|x| base::average(&x)).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sma_test() {
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let s = &array[..];

        assert_eq!(sma(s, 3), [0.3333333333333333, 1.0, 2.0, 3.0, 4.0]);
    }
}
