use std::{thread, time::Duration, time::Instant};

use windows::Win32::Media::Multimedia;

mod winsleep;

const ITERATIONS: u32 = 10;

fn test_windows_sleep(dur: Duration) {
    println!("expected: {} us",dur.as_micros());
    for _i in 1..ITERATIONS {
        let start = Instant::now();
        winsleep::sleep(dur);
        println!(
            "  actual: {} us",
            start.elapsed().as_micros()
        );
    }
}

fn test_thread_sleep(dur: Duration) {
    println!("expected: {} us",dur.as_micros());
    for _i in 1..ITERATIONS {
        let start = Instant::now();
        thread::sleep(dur);
        println!(
            "  actual: {} us",
            start.elapsed().as_micros()
        );
    }
}

fn main() {
    let (min, max) = winsleep::get_timecaps();
    println!("min {}, max {}", min, max);

    println!("=== thread::sleep ===");
    test_thread_sleep(Duration::from_millis(10));
    test_thread_sleep(Duration::from_millis(3));
    test_thread_sleep(Duration::from_millis(2));
    test_thread_sleep(Duration::from_millis(1));
    test_thread_sleep(Duration::from_micros(500));
    test_thread_sleep(Duration::from_micros(250));
    test_thread_sleep(Duration::ZERO);
    
    println!("=== windows_sleep ===");
    test_windows_sleep(Duration::from_millis(10));
    test_windows_sleep(Duration::from_millis(3));
    test_windows_sleep(Duration::from_millis(2));
    test_windows_sleep(Duration::from_millis(1));
    test_windows_sleep(Duration::from_micros(500));
    test_windows_sleep(Duration::from_micros(250));
    test_windows_sleep(Duration::ZERO);

    unsafe { Multimedia::timeBeginPeriod(1); }

    println!("=== thread::sleep after beginTimePeriod ===");
    test_thread_sleep(Duration::from_millis(10));
    test_thread_sleep(Duration::from_millis(3));
    test_thread_sleep(Duration::from_millis(2));
    test_thread_sleep(Duration::from_millis(1));
    test_thread_sleep(Duration::from_micros(500));
    test_thread_sleep(Duration::from_micros(250));
    test_thread_sleep(Duration::ZERO);

    println!("=== windows_sleep after beginTimePeriod ===");
    test_windows_sleep(Duration::from_millis(10));
    test_windows_sleep(Duration::from_millis(3));
    test_windows_sleep(Duration::from_millis(2));
    test_windows_sleep(Duration::from_millis(1));
    test_windows_sleep(Duration::from_micros(500));
    test_windows_sleep(Duration::from_micros(250));
    test_windows_sleep(Duration::ZERO);

    unsafe { Multimedia::timeEndPeriod(1); }
}
