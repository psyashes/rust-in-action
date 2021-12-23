fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    println!("base: {}", base);
    let large_n = (n as u32) << 15;
    println!("large_n: {}", large_n);
    let f32_bits = base | large_n;
    println!("f32_bits: {}", f32_bits);
    let m = f32::from_bits(f32_bits);
    println!("m: {}", m);
    2.0 * (m - 0.5)
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
