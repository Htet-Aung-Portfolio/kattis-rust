/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region importing necessary libraries */
use std::io;
/*#endregion */

fn closer_number(target: i32, larger_number: i32, smaller_number: i32) -> i32{
    //Display the closer number but if tie display the bigger one
    if (larger_number - target) <= (target-smaller_number) {
        larger_number
    }else{
        smaller_number
    }
}

fn main() {

    /*#region init */
    let input = io::stdin();
    /*#endregion */

    /*#region input */
    let mut original_price_string = String::new();
    input.read_line(&mut original_price_string);
    let original_price = original_price_string.trim().parse::<i32>().unwrap();
    /*#endregion */

    /*#region output */
    if original_price < 100 {
        print!("{}", 99);
    }else{
        //There are probably built-in functions that I could just used directly but I will just go for string operation
        let mut base_number_string = original_price_string.trim().to_owned();

        //remove the last the two digits
        base_number_string.remove(base_number_string.len() - 1);
        base_number_string.remove(base_number_string.len() - 1);
    

        //change the digit to 99
        let larger_number = ((&base_number_string).to_owned() + "99").parse::<i32>().unwrap();

        let base_number_minus_1 : i32 = base_number_string.parse::<i32>().unwrap() - 1;
        let mut smaller_number: i32 = 0;
        if base_number_minus_1 == 0 {
            //only possible for 100-200 values
            smaller_number = 99;
        }else{
            //-1 the digit on the hundred area and add the two last digit
            smaller_number = (base_number_minus_1.to_string() + "99").to_string().parse::<i32>().unwrap();
        }

        //finally display the closer value
        print!("{}", closer_number(original_price, larger_number, smaller_number))

    }
    /*#endregion */
}
