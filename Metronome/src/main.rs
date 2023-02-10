//<summary>
//Author : Htet Aung Hlaing
//</summary>

use std::io;

fn main() {

    let input = io::stdin();
    
    let mut input_string = String::new();
    input.read_line(&mut input_string);
    let input_value = input_string.trim().parse::<f64>().unwrap();

    println!("{:.2}", input_value/(4 as f64));
}
