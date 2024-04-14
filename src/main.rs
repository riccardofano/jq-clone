use std::io::{stdin, BufRead};

use jq_clone::apply_filter;

fn main() {
    let filter = std::env::args().nth(1);

    let mut buf = String::new();
    let _ = stdin()
        .lock()
        .read_line(&mut buf)
        .expect("Could not read stdin");

    let pretty_string = apply_filter(&buf, filter.as_deref()).expect("Could apply filter");

    println!("{pretty_string}");
}
