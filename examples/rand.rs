use rand::Rng;

fn main() {
    loop {
        if let Ok(ins) = bad64::decode(rand::thread_rng().gen(), rand::thread_rng().gen()) {
            print!("{:x?}", ins.operation());
            for n in 0..ins.num_operands() {
                print!(" {:x?}", ins.operand(n).unwrap());
            }
            println!("");
        }
    }
}
