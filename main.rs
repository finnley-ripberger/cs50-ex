use std::{io, string};

fn main() {
    println!("How high is the tower?");
    let mut height =String::new();
    io::stdin()
    .read_line(&mut height)
    .expect("failed to read line");
let  trueheight: i32 = height.trim().parse().expect("Input not a number");
let mut trueheight2: i32 = height.trim().parse().expect("Input not a number");
let mut startheight = 0;
//let mut loopcount = 0;
while trueheight >= startheight
    {let output = "#".repeat(startheight as usize);
    let spacecount = " ".repeat(trueheight2 as usize);
    println!("{} {} {}", spacecount, output, output);
    startheight += 1;
    trueheight2 -= 1;
}
}
