/*
<summary>
    Author : Htet Aung Hlaing
</summary>
 */

use std::io;

fn main() {
    let input = io::stdin();

    let mut input_string = String::new();
    input.read_line(&mut input_string);

    print!("{}", input_string.trim().replace("e", "ee"));

}
