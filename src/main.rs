extern crate dft;
use dft::{Operation, Plan};
use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main(){
    test_fft();
}

fn test_fft(){
    /*
    Setup
    */
    let args: Vec<String> = env::args().collect();
    let mut output_file = File::create("fft_output.txt").unwrap();
    let trials = calc_trials(args[1].parse::<usize>().unwrap());

    write!(
        output_file,
        "FFT TEST on matrix sizes: {:?}\n",
        trials
    ).unwrap();
    /*
    */
    

    for sz in trials{
        println!("running fft on size: {}", sz);
        let mut data = init_arr(sz);
        let plan = Plan::new(Operation::Forward, sz);


        /*
        Perform fft forwards
        */
        let tracer = dft::transform(&mut data, &plan);
        /*
        */

        /*
        Access fields
        */
        let _stack = *tracer.stack;
        let _freq_map = *tracer.freq_map;
        let dmd = *tracer.dmd;
        let trace = *tracer.trace;
        /*
        */

        /*
        Calculations
        */
        let trace_len: f32 = trace.len() as f32;
        let time_cmplx: f32 = sz as f32 * (sz as f32).ln();

        let dmd_per_trace: f32= dmd as f32 / trace_len;
        let dmd_per_time: f32 = dmd as f32 / time_cmplx;
        /*
        */
        
        /*
        Write to file
        */
        writeln!(
            output_file,
            "{}\t{}\t{}",
            sz,dmd_per_trace, dmd_per_time
        ).unwrap();
        /*
        */
    }
}

// fn setup(args: Vec<String>) -> (usize, Vec<f32>, File, Vec<usize>){   
//     let data = 
//     let trials = calc_trials(args[1].parse::<usize>().unwrap());

    
//     let output_file = File::create("fft_output.txt").unwrap();
//     (sz,data,output_file,trials)
// }

fn calc_trials(upper_bound: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut n = 2;
    while n <= upper_bound{
        res.push(n);
        n *= 2;
    }
    res
}

fn init_arr(n: usize) -> Vec<f32>{
    let mut res = Vec::new();
    for i in 0..n{
        res.push(i as f32);
    }
    res
}