//<summary>
// Author : Htet Aung Hlaing
//</summary>

use std::{io};

//There could be better ways to impl this but yea, just trying out enums in Rust with this problem
#[derive(PartialEq,Clone, Copy, Debug)]
enum Direction{
    North,
    East,
    South,
    West
}

impl Direction{

    /*#region Base Functions to get directions */
    fn get_opposite(&self) -> Self{
        use Direction::*;
        match *self{
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    fn get_left(&self) -> Self{
        use Direction::*;
        match *self{
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn get_right(&self) -> Self{
        use Direction::*;
        match *self{
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    /*#endregion */

    /*#region check directions and return boolean value */
    fn is_opposite(&self, target: Direction) -> bool {
        //print!("{:?} {:?} is_opposite", self.to_owned(), target.to_owned());
        self.get_opposite() == target
    }

    fn is_left(&self, target: Direction) -> bool{
        //print!("{:?} {:?} is_left", self.to_owned(), target.to_owned());
        self.get_left() == target
    }

    fn is_right(&self, target: Direction) -> bool{
        //print!("{:?} {:?} is_right", self.to_owned(), target.to_owned());
        self.get_right() == target
    }
    /*#endregion */

    /*#region fromSting Method */
    fn fromstr(str_version: &str) -> Self{
        use Direction::*;
        match str_version{
            "North" => North,
            "East" => East,
            "South" => South,
            "West" => West,
            _ => North
        }
    }
    /*#endregion */
}

//Created multiple functions so as to keep it more distributed? (idk the terms)
/*#region to check if the conditions are satisfied*/

//Check condition 1 only
fn check_condition1(from: Direction, to: Direction, other: Direction) -> bool {
    from.is_opposite(to) && from.is_right(other)
}

//Check condition 2 only
fn check_condition2(from: Direction, to: Direction, other: Direction) -> bool{
    from.is_left(to) && (from.is_opposite((&other).to_owned()) || from.is_right(other))
}

//Check both condition
fn check_condition(from: Direction, to: Direction, other: Direction) -> bool{
    check_condition1((&from).to_owned(), (&to).to_owned(), (&other).to_owned()) || check_condition2(from, to, other)
}
/*#endregion */
fn main() {
    let input = io::stdin();

    /*#region getting input */
    let mut input_string = String::new();
    input.read_line(&mut input_string);
    let values = input_string.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
    /*#endregion */

    /*#region Changing str into enum for operations*/
    let from = Direction::fromstr(&values[0].to_owned());
    let to = Direction::fromstr(&values[1].to_owned());
    let other = Direction::fromstr(&values[2].to_owned());
    /*#endregion */

    //println!("{:?} {:?} {:?}", from.to_owned(), to.to_owned(), other.to_owned());
    //Dropping since it is not used anymore
    drop(values);
    
    if check_condition(from, to, other) {
        print!("Yes");
    }else{
        print!("No");
    }

}
