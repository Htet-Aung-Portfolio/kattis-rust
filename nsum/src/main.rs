//<summary>
//  Author : Htet Aung Hlaing
//</summary>

use std::io;

fn main() {

    //input
    let input = io::stdin();

    //not even need to consider
    let mut number_of_numbers_string: String = String::new();
    input.read_line(&mut number_of_numbers_string); //<- Waste of Space
    
    let mut numbers_string: String = String::new();
    input.read_line(&mut numbers_string);
    println!("{}", numbers_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().sum::<i32>());

}
