extern crate dft;
use dft::{Operation, Plan};


fn main(){
    test_fft();
}

fn test_fft(){
    let sz = 8192;
    let plan = Plan::new(Operation::Forward, sz);
    //let rplan = Plan::new(Operation::Inverse, 16);
    let mut data = init_arr(sz);
    //println!("{:?}", data);
    let tracer = dft::transform(&mut data, &plan);
    //println!("{:?}", data);
    println!("{}", tracer.dmd);
    // dft::transform(&mut data, &rplan);
    // println!("{:?}", data);
}

fn init_arr(n: usize) -> Vec<f32>{
    let mut res = Vec::new();
    for i in 0..n{
        res.push(i as f32);
    }
    res
}