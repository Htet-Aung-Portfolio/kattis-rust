/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region importing necessary libraries */
use std::io;
/*#endregion */

//check quadrant with this function
fn get_quadrant(x: i32, y:i32) -> i32{
    if(y > 0){
        if x > 0 {
            1
        }else{
            2
        }
    }else{
        if x > 0 {
            4
        }else{
            3
        }
    }
}

fn main() {
    let input = io::stdin();

    /*#region init */
    let mut x_string = String::new();
    let mut y_string = String::new();
    /*#endregion */

    /*#region input */
    input.read_line(&mut x_string);
    input.read_line(&mut y_string);
    /*#endregion */

    /*#region parsing to i32 */
    let x = x_string.trim().parse::<i32>().unwrap();
    let y = y_string.trim().parse::<i32>().unwrap();
    /*#endregion */

    print!("{}", get_quadrant(x, y));
}
