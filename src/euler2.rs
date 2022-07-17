pub fn print_sum_of_sequence() {
    // First we define a variable to hold the vector that will contain the fibonacci sequence, currently it has 0 and 1 since these numbers are the numbers that "drive" the entire sequence
    let mut sequence = vec![0,1];
    
    /* 
       Then we create a while loop that runs while the last item in the vector + the second to last item in the vector are smaller than 4,000,000
       Note: the program checks if the two last items' sum is less than 4,000,000 to prevent the sequence from including a number bigger than 4,000,000 by accident 
    */
    while sequence[sequence.len()-1] + sequence[sequence.len()-2] < 4_000_000 {
        // While the loop runs the program will add to the 'sequence' vector the sum of the two last items in the vector
        sequence.push(sequence[sequence.len()-1]+sequence[sequence.len()-2]);
    }
    
    // Now, let us create a new variable which will include all the even numbers in the fibonacci sequence
    let mut even_sequence = vec![];
    
    // For that task we have created a for loop to go over each of the members of the fibonacci sequence
    for number in sequence {
        // If the number is even, then add it to the 'even_sequence' vector
        if number % 2 == 0 {
            even_sequence.push(number);
        }
    }
    
    // Finally we need to create a variable to contain the sum of the entire even sequence
    // For that we have taken the 'even_sequence' vector and made it an iterable which lets us use the .sum() method that adds all the members of the iterable together
    // Note: I have indicated a type *manually* since the compiler can't decide which type the code will result in
    let sum: i32 = even_sequence.iter().sum();
    
    println!("The sum of all the even numbers that don't exceed 4,000,000 in the fibonacci sequence is: {}", sum)
}

// That's it!