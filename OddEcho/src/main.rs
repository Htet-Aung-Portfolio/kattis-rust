/*
<summary>
    Author : Htet Aung Hlaing
</summary>
 */

use std::io;

fn is_odd(number: i32) -> bool {
    number % 2 != 0
}

fn main() {
    let input = io::stdin();

    let mut count_input_string = String::new();
    input.read_line(&mut count_input_string);
    let count = count_input_string.trim().parse::<i32>().unwrap();

    let mut output_vec:Vec<String> = Vec::new();

    let mut loop_index = 1;
    while (loop_index <= count) {


        let mut input_string = String::new();
        input.read_line(&mut input_string);

        if is_odd(loop_index.to_owned()) {
            output_vec.push(input_string.trim().to_string());
            //println!("{}", input_string.trim());
        }

        loop_index += 1;
    }

    for output in output_vec {
        println!("{}", output.trim());
    }
}
