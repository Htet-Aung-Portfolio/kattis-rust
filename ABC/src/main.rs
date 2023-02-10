/*#region Importing Necessary Libraries Here */
use std::io::{self, Read};
/*#endregion */

fn main() {
    
    let mut numbers = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut numbers);

    let mut numbers_split = numbers.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    numbers_split.sort_by(|a,b| a.cmp(b));

    fn option_from_char(target: char)->usize{
        match target{
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => 0
        }
    }

    let mut abc_order = [0; 3];
    stdin.read(&mut abc_order);

    print!("{:?} ", numbers_split[option_from_char(abc_order[0] as char)]);
    print!("{:?} ", numbers_split[option_from_char(abc_order[1] as char)]);
    print!("{:?}",  numbers_split[option_from_char(abc_order[2] as char)]);
}
