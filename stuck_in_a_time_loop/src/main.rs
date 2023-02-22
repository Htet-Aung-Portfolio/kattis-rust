///<summary>
///     Author : Htet Aung Hlaing
/// </summary>

use std::io;

fn main() {

    let input = io::stdin();

    let mut input_string = String::new();
    input.read_line(&mut input_string);

    let amount_of_times = input_string.trim().parse::<i32>().unwrap();


    for num in 0..amount_of_times
    {
        println!("{} {}", num + 1, "Abracadabra");
    }

}
