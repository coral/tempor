use std::thread;
use std::time::Duration;
use tempor::Tapper;

fn main() {
    // 4 is number of taps until it averages
    // Duration is the cooldown from where it resets the counter
    let mut tapper = Tapper::new(4, Duration::from_secs(3)).unwrap();

    loop {
        match tapper.tap() {
            Some(v) => {
                println!("BPM: {}", v);
            }
            None => {}
        };

        //119-120 BPM!
        thread::sleep(Duration::from_millis(500));
    }
}
