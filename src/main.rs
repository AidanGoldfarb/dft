extern crate dft;
use dft::{Operation, Plan};
use std::env;


fn main(){
    test_fft();
}

fn test_fft(){
    let args: Vec<String> = env::args().collect();

    let sz = args[1].parse::<usize>().unwrap();
    let plan = Plan::new(Operation::Forward, sz);
    //let rplan = Plan::new(Operation::Inverse, 16);
    let mut data = init_arr(sz);
    //println!("{:?}", data);
    let tracer = dft::transform(&mut data, &plan);
    //println!("{:?}", data);
    println!("{}", *tracer.dmd / (sz as f32 * (sz as f32).ln()));
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