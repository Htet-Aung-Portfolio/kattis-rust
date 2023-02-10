/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region Import Area */

use std::io;

/*#endregion */

//function to check if the number is even
fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        true
    }else{
        false
    }
}

fn main() {
    let input= io::stdin();
    
    /*#region input */
    let mut number_string = String::new();
    input.read_line(&mut number_string);
    let mut number = number_string.trim().parse::<i32>().unwrap();
    /*#endregion */

    /*#region check the number*/
    if is_even(number) {
        print!("Bob");
    }else{
        print!("Alice");
    }
    /*#endregion */
}
