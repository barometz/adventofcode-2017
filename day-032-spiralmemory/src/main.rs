extern crate day_032_spiralmemory;

fn main() {
    for square in 1.. {
        let sum = day_032_spiralmemory::adjacent_sum(square);
        if sum > 325_489 {
            println!("Result: {}", sum);
            break;
        }
    }
}
