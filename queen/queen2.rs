use std::time::{SystemTime, UNIX_EPOCH};

const N:i32 = 13;

fn clock_realtime() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    return since_the_epoch.as_millis() as i64;
}

fn array_check(array: &[i32], row: i32) -> bool {
    if row == 0 {
        return true;
    }

    let x0 = array[row as usize];
    for (y, &x) in (&array[..row as usize]).iter().enumerate() {
        if x == x0 {
            return false;
        }
        else if (x - x0).abs() == row - y as i32 {
            return false;
        }
    }

    true
}

fn search(array: &mut [i32], row: i32) -> i32 {
    let mut found = 0;
    assert!((row as usize) < array.len());

    for i in 0..array.len() {
        array[row as usize] = i as i32;

        if array_check(array, row) {
            if row == N - 1 {
                found += 1;
            }   else {
                found += search(array, row + 1);
            }
        }
    }

    found
}

fn queen() -> i32 {
    let mut array = [0; N as usize];
    search(&mut array, 0)
}

fn main() {
    let ts = clock_realtime();
    let found = queen();
    let dt = clock_realtime() - ts;
    println!("found={} time={} ms", found, dt);
}



