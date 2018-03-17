fn panic_crash_and_burn() {
    panic!("crash and burn")
}

fn panic_out_of_range_index() {
    let v = vec![1, 2, 3];
    v[99];
}

fn main() {
    // panic_crash_and_burn();
    panic_out_of_range_index();
}
