/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region importing necessary libraries */
use std::io::{self, Read};
/*Eendregion */

fn advertisement_result(without_advertisement: i32, with_advertisement: i32) -> String{
    if(without_advertisement > with_advertisement){
        "do not advertise".into()
    }else{
        if(without_advertisement == with_advertisement){
            "does not matter".into()
        }else{
            "advertise".into()
        }
    }
}
fn main() {

    /*#region init */
    let input = io::stdin();
    let mut result_vector : Vec<String> = Vec::new();
    /*#endregion */

    /*#region getting number of test cases */
    let mut number_of_test_cases_string = String::new();
    input.read_line(&mut number_of_test_cases_string);
    let number_of_test_cases = number_of_test_cases_string.trim().parse::<i32>().unwrap();
    /*#endregion */

    /*#region input loop */
    let mut loop_index = 0;
    while(loop_index < number_of_test_cases){
        //println!("Hello World!");
        
        /*#region the input for this particular test case */
        let mut data_string = String::new();
        input.read_line(&mut data_string);
        let data_vec = data_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        /*#endregion */

        let non_advertise_profit = data_vec[0];
        
        let with_advertise_revenue = data_vec[1];
        let advertise_cost: i32 = data_vec[2];

        let with_advertise_profit = with_advertise_revenue - advertise_cost;
        drop(data_vec);
        result_vector.push(advertisement_result(non_advertise_profit, with_advertise_profit));

        loop_index += 1;
    }
    /*#endregion*/

    /*#region Output Loop */
    loop_index = 0;
    while(loop_index < number_of_test_cases){
        println!("{}", result_vector[(&loop_index).to_owned() as usize]);
        loop_index += 1;
    }
    /*#endregion */
}
