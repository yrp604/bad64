use rand::Rng;

fn main() {
    loop {
        if let Ok(decoded) = bad64::decode(rand::rng().random::<u32>(), rand::rng().random::<u64>())
        {
            println!("{:x}: {}", decoded.opcode(), decoded);
        }
    }
}
