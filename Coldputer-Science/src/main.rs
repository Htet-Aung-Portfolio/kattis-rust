/*
<summary>
    Author: Htet Aung Hlaing
    Created Date: 8th Feb, 2023
<summary>
*/

/* #region Importing Necessary Libraries */
use std::{io};
/* #endregion */

fn main() {

    let mut stdin = io::stdin();
    
    /*#region first line */
    let mut number_of_temperatures_string = String::new();
    stdin.read_line(&mut number_of_temperatures_string);
    let number_of_temperatures = number_of_temperatures_string.trim().parse::<i32>().unwrap();
    /*#endregion */

    /*#region second line */
    let mut temperatures = String::new();
    stdin.read_line(&mut temperatures);
     /*#endregion */


     /*#region output */
     //using array functions map and filter to directly get the value we want
    let values_less_than_zero = temperatures.trim().split_whitespace().take(number_of_temperatures as usize).map(|x| x.parse::<i32>().unwrap()).filter(|x| x < &0).collect::<Vec<i32>>().len(); 
    print!("{}", values_less_than_zero);
    /*#endregion */

}
