/*
<summary>
    Author : Htet Aung Hlaing
</summary> */

use std::io;

fn is_divisible(target: i32, divise_number: i32) -> bool {
    if target % divise_number == 0 {
        true
    }else{
        false
    }
}

fn display_answer(condition1 : bool, condition2 : bool, number: i32) {
    if condition1.to_owned() && condition2.to_owned() {
        println!("FizzBuzz");
    }else{
        if condition1.to_owned() {
            println!("Fizz");
        }else if condition2.to_owned() {
            println!("Buzz");
        }else{
            println!("{}", number);
        }
    }
}

fn main() {
    let input = io::stdin();

    let mut input_string = String::new();
    input.read_line(&mut input_string);
    let input_variables = input_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let number1 = input_variables[0].to_owned();
    let number2 = input_variables[1].to_owned();
    let till = input_variables[2].to_owned();

    let mut loop_index = 1;

    while loop_index <= till {

        display_answer(is_divisible(loop_index.to_owned(), number1.to_owned()), is_divisible(loop_index.to_owned(), number2.to_owned()), loop_index.to_owned());
        loop_index += 1;
    }
}
