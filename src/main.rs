//////////////////////////////////
//       crab2 simulator        //
//////////////////////////////////

use rand::Rng;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp_file = "last_run.txt";

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    if let Ok(contents) = fs::read_to_string(timestamp_file) {
        if let Ok(last_run) = contents.trim().parse::<u128>() {
            if now.saturating_sub(last_run) < 5000 {
                return;
            }
        }
    }

    let _ = fs::write(timestamp_file, now.to_string());

    let mut a: u64 = 0;
    let mut rng = rand::thread_rng();

    loop {
        let roll = rng.gen_range(1..=100);

        if roll <= 90 {
            a += 1;
        } else {
            break;
        }
    }

    if a == 0 {
        println!("0 crab");
    } else {
        println!("{}", "🦀".repeat(a as usize));
    }
}
