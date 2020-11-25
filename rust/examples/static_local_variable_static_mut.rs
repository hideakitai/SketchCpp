fn print_and_count_number() {
    static mut NUMBER_MUT: u32 = 5u32;
    println!("{}", unsafe { NUMBER_MUT });
    unsafe {
        NUMBER_MUT += 1;
    }
}

fn main() {
    print_and_count_number();
    print_and_count_number();
    print_and_count_number();
}
