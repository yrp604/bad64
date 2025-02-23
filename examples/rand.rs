use rand::Rng;

fn main() {
    loop {
        if let Ok(decoded) = bad64::decode(rand::thread_rng().r#gen(), rand::thread_rng().r#gen()) {
            println!("{:x}: {}", decoded.opcode(), decoded);
        }
    }
}
