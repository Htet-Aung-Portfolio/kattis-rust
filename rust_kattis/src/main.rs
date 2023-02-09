/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region Importing Necessary Libraries */
use std::io;
/*#endregion */
fn main() {

    /*#region init */
    let mut input_string = String::new();
    let input = io::stdin();
    let numbers: Vec<i32> = Vec::new();
    /*endregion */

    /*#region input */
    input.read_line(&mut input_string);
    let numbers = input_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("{}", numbers[0] * numbers[1] * numbers[2]);
    /*#endregion */
}
