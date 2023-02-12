/*<summary>
    Author : Htet Aung Hlaing
</summary>*/

use std::io;

/*
This Pythagorus function does not work because it does not account for 1 1 1 which seems to be true but in fact we all know it is wrong

fn check_and_display_simple(base: i32, perpendicular: i32, hypotenuse: i32){
    let base_squared = base * base;
    let perpendicular_squared = perpendicular * perpendicular;
    let hypotenuse_squared = hypotenuse * hypotenuse;

    if(base_squared + perpendicular_squared == hypotenuse_squared) {
        println!("right")
    }else{
        println!("wrong")
    }
}
*/

fn get_angle_from_sin(sin_value: f32)-> f32{
    sin_value.asin().to_degrees()
}

fn check(base: i32, perpendicular: i32, hypotenuse: i32) -> bool{
    //using sin cos tan, we can basically regard that
    //if there are 3 Angles, A, B, and C
    //if B is the right triangle
    //A + C must be 90
    //so we will try to get the angle of A and C through sin
    //and plus them to see if it's 90
    let angle1 : f32 = base as f32/hypotenuse as f32;
    let angle2 : f32 = perpendicular as f32/ hypotenuse as f32;
    if ((90 as f32 - (get_angle_from_sin((&angle1).to_owned()) + get_angle_from_sin((&angle2).to_owned()))).abs() < 0.001 as f32) && base != 0 && perpendicular != 0 && hypotenuse != 0 {
        true
    }else{
        false
    }
}
fn main() {
    let input = io::stdin();
    let mut output_vec: Vec<bool> = Vec::new();

    loop{
        let mut input_string = String::new();
        input.read_line(&mut input_string);
        let numbers = input_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if(numbers[0].to_owned() == 0 && numbers[1].to_owned() == 0 && numbers[2].to_owned() == 0) {
            break;
        }

        let base = numbers[0].to_owned();
        let perpendicular = numbers[1].to_owned();
        let hypotenuse = numbers[2].to_owned();
        drop(numbers);

        output_vec.push(check(base, perpendicular, hypotenuse));
    }

    for value in output_vec.iter() {
        if *value {
            println!("right");
        }else{
            println!("wrong");
        }   
    }
}
