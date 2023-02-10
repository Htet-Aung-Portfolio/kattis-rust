/*
<summary>
    Author : Htet Aung  Hlaing

    So the program is asking us to check if the amount of '.' is the same between "the stars"
    So what I will do is that I will remove the first occurence of "*" and the number of "." between it and the second "*"
    if there is any dots that is left after this operation, we can consider this to be not even
</summary>
*/

/*#region import */
use std::io;
/*#endregion */

/*#region functions */
fn display_result(index: i32, is_even: bool){

    let result: String = if is_even { "EVEN".into() } else { "NOT EVEN".into() };
    println!("{} {}", index, result)
}

fn get_pattern(line: String) -> String{

    let mut mut_line = line.to_owned();
    let mut pattern: String = "*".into();

    let first_star_result = mut_line.chars().position(|character| character == '*');
    
    if(first_star_result.is_none()){
        pattern
    }else{
        let first_star_index = first_star_result.unwrap();
        mut_line.remove(first_star_index);

        let second_star_result = mut_line.chars().position(|character| character == '*');
        if(second_star_result.is_none()){
            pattern
        }else{
            let second_star_index = second_star_result.unwrap();
            let number_of_white_dots = (second_star_index as i32) - first_star_index as i32;
            
            let mut index = 0;
            while(index < number_of_white_dots){
                pattern += ".";
                index += 1;
            }
            pattern
        }
    }
}

fn is_pattern_consistent(original: String, pattern: String) -> bool{
    let mut original_mut = original.to_owned();

    //if after replacement it DOES NOT contain any '.' the pattern can be consistent
    let replaced = original_mut.replace(&pattern, "");
    (replaced == "*" || replaced.is_empty()) && !replaced.contains('.')
}
/*#endregion */

fn main() {

    /*#region init */
    let mut is_even_array = Vec::<bool>::new();;
    let input = io::stdin();
    /*#endregion */

    /*#region insert */
    loop {
        let mut line = String::new();
        input.read_line(&mut line);
        line = line.trim().into();

        if line == "END" {
            break;
        }else{
            let pattern_to_remove: String = get_pattern(line.to_owned());
            is_even_array.push(is_pattern_consistent(line, pattern_to_remove));
        }
    }
    /*#endregion */

    let mut index = 1;
    /*#region Display */
    for boolean_element in is_even_array {
        display_result((&index).to_owned(), boolean_element);
        index += 1;
    }
    /*#endregion */
}
