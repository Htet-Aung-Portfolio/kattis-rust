/*<summary>
    Author: Htet Aung Hlaing
</summary>*/

use std::io;
use std::fmt;

fn main() {
    let input = io::stdin();

    let mut number_of_test_cases_string = String::new();
    input.read_line(&mut number_of_test_cases_string);
    let number_of_test_cases = number_of_test_cases_string.trim().parse::<i32>().unwrap();

    let mut output_string_array : Vec<String> = Vec::new();

    let mut loop_index = 0;
    while(loop_index < number_of_test_cases){

        let mut case = String::new();
        input.read_line(&mut case);
        let value = case.trim().parse::<i32>().unwrap();

        if value % 2 == 0{
            //Even
            output_string_array.push(fmt::format(format_args!("{} is even", value)));
        }else{
            output_string_array.push(fmt::format(format_args!("{} is odd", value)));
        }   
        loop_index += 1;
    }

    loop_index = 0;
    while(loop_index < number_of_test_cases){
        println!("{}", output_string_array[loop_index as usize]);
        loop_index += 1;
    }

}
