extern crate dft;
use dft::{Operation, Plan};


fn main(){
    test_fft();
}

fn test_fft(){
    let plan = Plan::new(Operation::Forward, 16);
    //let rplan = Plan::new(Operation::Inverse, 16);
    let mut data = vec![5.0; 16];
    println!("{:?}", data);
    let tracer = dft::transform(&mut data, &plan);
    println!("{:?}", data);
    println!("{}", tracer.dmd);
    // dft::transform(&mut data, &rplan);
    // println!("{:?}", data);
}