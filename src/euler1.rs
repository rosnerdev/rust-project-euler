// This program's intention is to solve problem #1 of project euler
fn main() {
    // First we define a variable to store the sum of all the multiples of 3 or 5 below 1000
    let mut sum = 0;

    // Then we initialize a "for" loop to iterate over each of the numbers below 1000
    for number in 1..1000 {
        // After that, we ask to the program to add the numbers which are, in fact multiples of 3 or 5 to the sum
        if number % 3 == 0 || number % 5 == 0 {
            sum += number;
        }
    }


    // Lastly we will print the result to the screen
    println!("The sum of all the multiples of 3 or 5 below 1000 is: {:?}", sum);
}

// Rust is so amazing! //
