use std::fs::File;
use std::io::prelude::*;

fn main(){
    //Find the sum of all even Fibonacci numbers below 4 million
    let mut a = 1;
    let mut b = 1;

    while b < 4000000 || b%2 != 0 {
        b += a;
        a = b - a;
    }
    let answer = (a - 1) / 2;
    let mut file = File::create("results.txt").expect("Could not create file!");
    let contents = format!("The answer to problem 2 is: {0}", answer);
    file.write(String::as_bytes(&contents)).expect("Could not write out content");

}
