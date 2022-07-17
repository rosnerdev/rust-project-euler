// I didn't want to make the main file to messy so I created separate files to contain the functions that solve the problems presented by Project Euler
// What the code below does is to import the two files as modules which we can extract functions from by using two colons or ::
mod euler1;
mod euler2;

fn main() {
    euler1::print_sum_of_multiples();
    euler2::print_sum_of_sequence();
}

// Run using the 'cargo run' command