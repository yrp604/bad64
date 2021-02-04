use rand::Rng;

fn main() {
    loop {
        if let Ok(ins) = bad64::decode(rand::thread_rng().gen(), rand::thread_rng().gen()) {
            print!("{:x?}", ins.operation());
            for o in ins.operands() {
                print!(" {:x?}", o);
            }
            println!("");
        }
    }
}
