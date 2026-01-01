// Example Rust code with detected AI shortcuts

fn main() {
    let input = "123";

    // unwrap() is the ultimate AI shortcut
    let _num: i32 = input.parse().unwrap();

    // todo! macro crashes at runtime
    // todo!("finish main function");
}

fn process_data(data: Option<String>) {
    // expect() is just unwrap with a message
    let s = data.expect("should be here");

    // cloning to avoid borrow checker complexity
    let _s2 = s.clone();
}

// Empty impl stub
struct Placeholder;
impl Placeholder {}
