/*<summary>
    Author: Htet Aung Hlaing
</summary> */

use std::io;

fn main() {
    let input = io::stdin();

    let mut number_of_cases_string = String::new();
    input.read_line(&mut number_of_cases_string);
    let number_of_cases = number_of_cases_string.trim().parse::<i32>().unwrap();

    let mut universities: Vec<String> = Vec::new();
    let mut output_value: Vec<String> = Vec::new();

    let mut loop_index = 0;
    while loop_index < number_of_cases {

        let mut school_status = String::new();
        input.read_line(&mut school_status);
        let school_status_vec: Vec<String> = school_status.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();

        let school_name = school_status_vec[0].to_owned();
        let team_name = school_status_vec[1].to_owned();

        let found_university = universities.iter().any(|x| x.to_string() == school_name.to_string());
        if !found_university {
            //First Occurence Add it
            output_value.push(school_status.trim().to_string());
            universities.push(school_name);
        }

        loop_index += 1;
    }

    loop_index = 0;
    while loop_index < 12{
        println!("{}", output_value[(&loop_index).to_owned() as usize]);
        loop_index += 1;
    }
}
