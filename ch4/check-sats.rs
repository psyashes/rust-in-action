#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(set_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let set_a = CubeSat { id: 0 };
    let set_b = CubeSat { id: 1 };
    let set_c = CubeSat { id: 2 };

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
