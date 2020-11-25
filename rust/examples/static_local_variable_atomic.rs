use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn print_and_count_numbers() {
    static N: AtomicUsize = AtomicUsize::new(5);
    println!("{}", N.fetch_add(1, Ordering::SeqCst));
}

fn main() {
    print_and_count_numbers();
    print_and_count_numbers();
    print_and_count_numbers();
}
