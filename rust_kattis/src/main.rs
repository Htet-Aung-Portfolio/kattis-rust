/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region Import Necessary Libraries */
use std::io::{self, stdin, Read};
/*#endregion */

/*#region Trait Model Building */
trait build_with_vector{
    fn new(input_vector: Vec<i32>) -> Self;
}

trait check_if_within{
    fn check_if_within(&mut self, call_object: &Call) -> bool;
}

trait unique_call_trait{
    fn new( Source: i32, Destination: i32)-> Self;

    fn contains(&self, call_object: &Call) -> bool;
}
/*#endregion */

/*#region build structs */
struct Call{
    Source : i32,           //Source of the call
    Destination : i32,      //Recipent of the call
    Start : i32,            //Call Start Time
    End : i32               //Duration of the call
}

struct UniqueCall{
    Source : i32,
    Destination : i32
}

impl unique_call_trait for UniqueCall{
    fn new(source: i32, destination: i32)-> Self {
        UniqueCall{
            Source : source,
            Destination: destination
        }
    }

    fn contains(&self, call_object: &Call) -> bool{
        self.Source == call_object.Source && self.Destination == call_object.Destination
    }
} 
//the interval to check
struct Interval{
    Start : i32,            //Start of the call    
    End : i32          //End of the call
}
/*#endregion */

/*#region trait new */
impl build_with_vector for Call{
    fn new(input_vector: Vec<i32>) -> Self {
        Call{
            Source: input_vector[0],                    //Source of the call
            Destination: input_vector[1],               //Destination of the call
            Start: input_vector[2],                     //Start
            End: input_vector[2] + input_vector[3]      //Start + Duration
        }
    }
}

impl build_with_vector for Interval{
    fn new(input_vector: Vec<i32>) -> Self{
        Interval{
            Start: input_vector[0],
            End: input_vector[0] + input_vector[1]
        }
    }
}

impl check_if_within for Interval{
    fn check_if_within(&mut self, call_object: &Call) -> bool {
        //println!("{} {}", call_object.Start, call_object.End);
        check_if_int_is_between(self.Start, call_object.Start, call_object.End) ||  check_if_int_is_between(self.End, call_object.Start, call_object.End) || 
        check_if_int_is_between(call_object.Start, self.Start, self.End) || check_if_int_is_between(call_object.End, self.Start, self.End)
    }
}

/*#endregion */

/*#region Some Custom Functions */
fn check_if_int_is_between(target: i32, start: i32, end: i32) -> bool{
    target > start && target < end
}
/*#endregion */

fn main() {
    let mut input = io::stdin();
    let mut result: Vec<i32> = Vec::new();

    loop{

        /*#region init */
        let mut count_of_calls_and_intervals_string = String::new();
        let mut calls: Vec<Call> = Vec::new();
        /*#endregion */

        /*#region Setting counts */
        input.read_line(&mut count_of_calls_and_intervals_string);
        let counts = count_of_calls_and_intervals_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        let mut number_of_calls = counts[0];
        let mut number_of_intervals = counts[1];

        if counts[0] == 0 && counts[1] == 0 {
            //Program should end here
            break;
        }
        /*#endregion */

        let mut call_loop_index = 0;
        while call_loop_index < number_of_calls {
            
            let mut call_object_string = String::new();
            input.read_line(&mut call_object_string);
            calls.push(Call::new(call_object_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()));
            call_loop_index += 1;
        }

        let mut interval_loop_index = 0;
        while interval_loop_index < number_of_intervals{

            let mut unique_calls: Vec<UniqueCall> = Vec::new();
            fn unique_call_check_and_insert(call_object: &Call, mut_unique_calls : &mut Vec<UniqueCall>) -> bool{
                let mut contains = false;
                for individual_unique_call in mut_unique_calls.iter() {
                    if individual_unique_call.contains(call_object) {
                        contains = true;
                        break;
                    }
                }
                
                if contains {
                    false
                }else{
                    mut_unique_calls.push(UniqueCall::new(call_object.Source, call_object.Destination));
                    true
                }
            }

            let mut interval_object_string = String::new();
            input.read_line(&mut interval_object_string);
            let mut interval_object = Interval::new(interval_object_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
            
            let mut eligible_calls = 0;
            for call_object in calls.iter() {
                if interval_object.check_if_within(call_object) {
                    unique_call_check_and_insert(call_object, &mut unique_calls);
                    //eligible_calls += 1;
                }
            }
            result.push(unique_calls.len() as i32);
            interval_loop_index += 1;
        }
        //println!("Program ending here");
    }

    for eligible_call in result{
        println!("{}", eligible_call);
    }
}
