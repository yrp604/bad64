fn main() {
    let start = std::time::Instant::now();
    let mut decoded = 0;

    for int in 0..=u32::MAX {
        let inst = bad64::decode(int, 0);

        if inst.is_ok() {
            decoded += 1;
        }

        if (int > 0) && (int & 0x07ff_ffff == 0) {
            let time = start.elapsed();
            let p = (int as f64) / (u32::MAX as f64) * 100.0;
            let rate = (int as f64) / u64::max(1, time.as_secs()) as f64;
            println!("checked {} words ({:.1}%) in {:.1?} ({:.0} words/sec)", int, p, time, rate);
        }
    }

    let time = start.elapsed();
    println!("decoded {} valid instructions in {:.1?}", decoded, time);
}
