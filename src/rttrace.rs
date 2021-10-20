use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
 	pub stack: Box<Vec<String>>,
	pub freq_map: Box<HashMap<usize,usize>>,
	pub dmd: Box<f32>,
    pub trace: Box<Vec<String>>,
}

// #[derive(Debug)]
// pub struct MMData{
//     pub a_b: Data,
//     pub c: Data,
//     pub temp: Data,
// }

pub fn init() -> Data {
	let stack = Box::new(Vec::new());
	let freq_map: Box<HashMap<usize,usize>> = Box::new(HashMap::new());
	let dmd = Box::new(0.0);
    let trace = Box::new(Vec::new());

	let d =  Data {
		stack,
		freq_map,
		dmd,
        trace,
	};
    d
}


pub fn trace(val: String, data: &mut Data){
    data.trace.push("n".to_string());
    if data.stack.contains(&val){ //resuse
        let position = data.stack.iter().position(|x| *x == val).unwrap();  //get position in stack
        if position == data.stack.len()-1{ //top of stack
            let freq = data.freq_map.entry(1).or_insert(0);
            *freq += 1;
        }
        else{
            let item = data.stack.remove(position);    //remove element and place at top
            data.stack.push(item);
            let temp_dist = data.stack.len()-position;
            
            let freq = data.freq_map.entry(temp_dist).or_insert(0);
            *freq += 1;

            *data.dmd += *freq as f32 * (temp_dist as f32).sqrt();
        }
    }
    else{
        data.stack.push(val);
    }
}