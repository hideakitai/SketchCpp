use std::cell::RefCell;

fn print_and_count_number() {
    thread_local!(
        pub static NUMBER_MUT: RefCell<u32> = RefCell::new(5u32)
    );

    NUMBER_MUT.with(|number| {
        let mut n = number.borrow_mut();
        println!("{}", *n);
        *n += 1;
    })
}

fn main() {
    print_and_count_number();
    print_and_count_number();
    print_and_count_number();
}
