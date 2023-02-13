/*<summary>
    Author: Htet Aung Hlaing
</summary> */

use std::io;
fn main() {
    let mut input = io::stdin();

    /*#region input */
    let mut input_string = String::new();
    input.read_line(&mut input_string);
    let input_counts = input_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    /*#endregion */

    /*#region output*/
    println!("{} {} {} {} {} {}", 1 - input_counts[0], 1 - input_counts[1], 2 - input_counts[2], 2 - input_counts[3], 2 - input_counts[4], 8 - input_counts[5])
    /*#endregion */
}
