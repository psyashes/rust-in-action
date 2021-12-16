#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(set_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let set_a = 0;
    let set_b = 1;
    let set_c = 2;

    let a_status = check_status(set_a);
    let b_status = check_status(set_b);
    let c_status = check_status(set_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // "waiting" ...
    let a_status = check_status(set_a);
    let b_status = check_status(set_b);
    let c_status = check_status(set_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
