// Rusty Technical Analysis

mod base;

use std::mem;
use std::slice;

#[no_mangle]
pub extern "C" fn sma_c(
        p_dest: *mut f64, p_src: *const f64, n: usize, period: usize) {

    let mut dest = unsafe {slice::from_raw_parts_mut(p_dest, n)};
    let src = unsafe {slice::from_raw_parts(p_src, n)};

    sma(dest, src, period);
}

fn sma(dest: &mut [f64], src: &[f64], period: usize) {
// The size of two slices must be same or panic.

    //mem::swap(dest,
    //        base::window_over_zero(src, period).to_vec().into_iter()
    //        .map(|x| base::average(&x))
    //        .collect::<Vec<_>>().get_mut(..).unwrap());
    // E0277: size for values of type [f64] cannot be known at compilation time
    // help: the trait `std::marker::Sized` is not implemented for `[f64]`
    // note: required by `std::mem::swap`

    let v = base::window_over_zero(src, period).to_vec().into_iter()
            .map(|x| base::average(&x)).collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
    }
}
