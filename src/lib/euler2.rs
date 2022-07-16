pub fn print_sum_of_sequence() {
    let mut sequence = vec![0,1];
    
    while sequence[sequence.len()-1] + sequence[sequence.len()-2] < 4_000_000 {
        sequence.push(sequence[sequence.len()-1]+sequence[sequence.len()-2]);
    }
    
    let mut even_sequence = vec![];
    
    for number in sequence {
        if number % 2 == 0 {
            even_sequence.push(number);
        }
    }
    
    let mut sum = 0;
    
    for number in even_sequence {
        sum += number;
    }
    
    println!("The sum of all the even numbers that don't exceed 4,000,000 in the fibonacci sequence is: {}", sum)
}