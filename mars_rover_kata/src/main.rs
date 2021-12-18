// Plateau ->  10x10 grid
// Rover -> position(X,Y), direction(N,S,E,W)
//          rotate L or R
//          Move
mod plateau;
mod rover;


use plateau::Plateau;
use rover::Rover;

fn main() {
    let p1 = Plateau::new();
    // println!("{:#?}", p1)
    let mut rover = Rover::new(String::from("1 2 N"));
    // println!("{:#?}",rover);
    let instructions = String::from("LMLMLMLMM");
    let final_result = rover.execute(instructions);
    println!("{}", final_result)
}
