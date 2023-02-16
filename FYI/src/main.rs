/*
<summary>
    Author : Htet Aung Hlaing
</summary> */

use std::io;

fn get_byte_string_index(byte_string: Vec<u8>, index: i32) -> char{
    byte_string[index as usize] as char
}

fn check_state(byte_string: &[u8]) -> bool {
    get_byte_string_index(byte_string.to_owned(), 0) == '5' &&     get_byte_string_index(byte_string.to_owned(), 1) == '5' &&     get_byte_string_index(byte_string.to_owned(), 2) == '5'
}

fn main() {
    let input = io::stdin();

    let mut input_string = String::new();
    input.read_line(&mut input_string);
    let byte_input_string = input_string.trim().as_bytes();

    if check_state(byte_input_string) {
        println!("1")
    }else{
        println!("0")
    }
}
