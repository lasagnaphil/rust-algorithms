extern crate rand;
use rand::Rng;

mod ds;
mod sorters;

pub use ds::heap::Heap;
pub use sorters::sorter::HeapSort;
pub use sorters::sorter::Sorter;

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (0..100).map(|x| rng.gen::<i32>()).collect();
    let sorter = HeapSort {};
    let sorted_numbers = sorter.sort(numbers);
    println!("{:?}", sorted_numbers);
}
