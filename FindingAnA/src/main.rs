///Author : Htet Aung Hlaing

use std::io;
fn main() {
     let input = io::stdin();

    /*#region input */
    let mut input_string = String::new();
    input.read_line(&mut input_string);
    input_string = input_string.trim().to_string();
    /*#endregion */

    /*#region Process and Output*/
    let skip_position: usize = (&input_string).chars().position(|x| x == 'a').unwrap() as usize;
    let amount_of_characters: usize = (&input_string).len() - &skip_position;

    println!("{}", input_string.chars().skip(skip_position).take(amount_of_characters).collect::<String>());
    /*#endregion */
}
