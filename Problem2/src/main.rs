use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // The sum of numbers from 1 to n is 1/2*n(n + 1) So the multiples of three are from 1*3 to 333*3
    // We factor out the three and get n = 333 and multiple the result of the formula by 3
    // Repeat this for 5 and then subtract by 15 which will be all the common multiples of 3 and 5
    let sum3 = 333 * (333 + 1) / 2 * 3;
    let sum5 = 199 * (199 + 1) / 2 * 5;
    let sum15 = 66 * (66 + 1) / 2 * 15;

    let mut file = File::create("results.txt").expect("Could not create file!");
    let contents = format!("The answer to problem 1 is: {0}", sum3 + sum5 - sum15);
    file.write(String::as_bytes(&contents))?;
    Ok(())
}
