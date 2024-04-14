use std::io::{stdin, Read};

use jq_clone::apply_filter;

fn main() {
    let filter = std::env::args().nth(1);

    let mut buf = String::new();
    let _ = stdin()
        .lock()
        .read_to_string(&mut buf)
        .expect("Could not read stdin");

    match apply_filter(&buf, filter.as_deref()) {
        Ok(string) => println!("{string}"),
        Err(e) => eprintln!("Failed to apply filters: {e:?}"),
    }
}
