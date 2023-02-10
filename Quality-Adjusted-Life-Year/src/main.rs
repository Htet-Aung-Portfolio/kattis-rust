/*
<summary>
    Author : Htet Aung Hlaing
</summary>
*/

use std::io;

fn main() {
    let input = io::stdin();

    let mut number_of_periods_string = String::new();
    input.read_line(&mut number_of_periods_string);
    let number_of_periods: i32 = number_of_periods_string.trim().parse().unwrap();

    let mut cumulative_quality_of_life: f64 = 0.0;
    let mut loop_index = 0;
    while (loop_index < number_of_periods){

        let mut input_string = String::new();
        input.read_line(&mut input_string);
        let decimals = input_string.trim().split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
    

        let mut quality_of_life: f64 = decimals[0];
        let mut time_period: f64 = decimals[1];
        cumulative_quality_of_life += quality_of_life * time_period;
        loop_index += 1;
    }

    println!("{:.3}", cumulative_quality_of_life);
}
