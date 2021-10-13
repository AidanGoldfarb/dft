// The implementation is based on:
// http://www.librow.com/articles/article-10
use crate::rttrace::{Data,init,trace};
use num_complex::Complex;
use num_traits::Float;

use {Operation, Plan, Transform};

impl<T> Transform<T> for [Complex<T>]
where
    T: Float,
{
    fn transform(&mut self, plan: &Plan<T>, tracer: &mut Data) {
        let n = self.len();
        assert!(n <= plan.n);
        rearrange(self, n, tracer);
        calculate(self, n, &plan.factors, tracer);
        if let Operation::Inverse = plan.operation {
            scale(self, n);
        }
        println!("done");
    }
}

impl<T> Transform<T> for Vec<Complex<T>>
where
    T: Float,
{
    #[inline(always)]
    fn transform(&mut self, plan: &Plan<T>, tracer: &mut Data) {
        Transform::transform(&mut self[..], plan, tracer)
    }
}

#[inline(always)]
fn calculate<T>(data: &mut [Complex<T>], n: usize, factors: &[Complex<T>], tracer: &mut Data)
where
    T: Float,
{
    let mut k = 0;
    let mut step = 1;
    while step < n {
        let jump = step << 1;
        for mut i in 0..step {
            while i < n {
                let j = i + step;
                unsafe {
                    trace("factors[".to_string() + &k.to_string()+&"]".to_string(), tracer);
                    trace("data[".to_string() + &j.to_string()+&"]".to_string(), tracer);
                    trace("factors[".to_string() + &k.to_string()+&"]".to_string(), tracer);
                    trace("product".to_string(), tracer);
                    let product = *factors.get_unchecked(k) * *data.get_unchecked(j);

                    trace("data[".to_string() + &i.to_string()+&"]".to_string(), tracer);
                    trace("product".to_string(), tracer);
                    trace("data[".to_string() + &j.to_string()+&"]".to_string(), tracer);
                    *data.get_unchecked_mut(j) = *data.get_unchecked(i) - product;

                    trace("factors[".to_string() + &i.to_string()+&"]".to_string(), tracer);
                    trace("product".to_string(), tracer);
                    trace("data[".to_string() + &i.to_string()+&"]".to_string(), tracer);
                    *data.get_unchecked_mut(i) = *data.get_unchecked(i) + product;
                }
                i += jump;
            }
            k += 1;
        }
        step <<= 1;
    }
}

#[inline(always)]
fn rearrange<T>(data: &mut [Complex<T>], n: usize, tracer: &mut Data) {
    let mut j = 0;
    for i in 0..n {
        if j > i {
            trace("data[".to_string() + &i.to_string()+&"]".to_string(), tracer);
            trace("data[".to_string() + &j.to_string()+&"]".to_string(), tracer);
            trace("data[".to_string() + &i.to_string()+&"]".to_string(), tracer);
            trace("data[".to_string() + &j.to_string()+&"]".to_string(), tracer);
            data.swap(i, j);
        }
        let mut mask = n >> 1;
        while j & mask != 0 {
            j &= !mask;
            mask >>= 1;
        }
        j |= mask;
    }
}

#[inline(always)]
fn scale<T>(data: &mut [Complex<T>], n: usize)
where
    T: Float,
{
    let factor = T::from(n).unwrap().recip();
    for value in data {
        *value = value.scale(factor);
    }
}
