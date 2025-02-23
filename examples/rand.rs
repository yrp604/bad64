use rand::Rng;

fn main() {
    loop {
        if let Ok(decoded) = bad64::decode(rand::thread_rng().r#gen(), rand::rng()) {
            println!("{:x}: {}", decoded.opcode(), decoded);
        }
    }
}
