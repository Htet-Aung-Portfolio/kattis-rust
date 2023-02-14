/*<summary>
    Author : Htet Aung Hlaing
</summary> */

use std::io;

fn change_base(original_number: i32, base: i32) -> String{
    let mut result_string = String::new();

    let mut number = original_number;
    while number > 0 {
        let remainder = number % base;
        result_string.insert(0, format!("{}", remainder).to_owned().as_bytes()[0] as char);

        number = number/ base;
    }

    result_string
}

fn main() {
    let input = io::stdin();

    let mut values = Vec::new();
    //get the number of cases
    let mut number_of_cases_string = String::new();
    input.read_line(&mut number_of_cases_string);
    let number_of_cases = number_of_cases_string.trim().parse::<i32>().unwrap();

    //Input and Process Loop
    let mut loop_index = 0;
    while loop_index < number_of_cases {

        let mut input_string = String::new();
        input.read_line(&mut input_string);
        let input_values = input_string.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();

        //The number represented by the source language
        let source_number = input_values[0].to_owned();
        //The numbers in the source langauge
        let source_language = input_values[1].to_owned();

        //The numbers in the alien language
        let alien_language = input_values[2].to_owned();
        //The number when converted into alien number
        let alien_number: String = String::new();

        let mut absolute_value: i32 = 0;

        /*#region Calculating absolute value */
        //we will get the number of characters in the string
        let mut source_number_power = source_number.len() as i32;
        let mut source_language_base = source_language.len() as i32;

        let mut loop_index_2 = 0;
        while source_number_power > 0 {
            
            absolute_value += (source_language.find(source_number.to_owned().as_bytes()[loop_index_2 as usize] as char).unwrap() as i32) * (source_language_base.to_owned().pow((source_number_power.to_owned() - 1) as u32));
            //println!("{}", absolute_value);

            loop_index_2 += 1;
            source_number_power -= 1;
        }
        /*#endregion */

        /*#region calculate with absolute value */
        //we will get the number of characters in the result string
        let mut alien_number_base = alien_language.len() as i32;
        let mut alien_value_in_string = change_base(absolute_value, alien_number_base);
        let mut value_to_display = String::new();

        loop_index_2 = 0;
        while(loop_index_2 < alien_value_in_string.len()){

            value_to_display.push(alien_language.to_owned().as_bytes()[(alien_value_in_string.to_owned().as_bytes()[loop_index_2 as usize] as char).to_string().parse::<i32>().unwrap() as usize] as char);

            loop_index_2 += 1;
        }
        values.push(value_to_display);
       //println!("Case #{}: {}",(&loop_index).to_owned() , value_to_display);
        /*#endregion */

        loop_index += 1;
    }

    loop_index = 1;

    for value in values.iter(){
        println!("Case #{}: {}",(&loop_index).to_owned() , value);
        loop_index += 1;
    }
}
