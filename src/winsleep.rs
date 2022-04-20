use std::mem;
use std::{ptr, time::Duration};

use lazy_static::lazy_static;

use windows::Win32::Foundation::{HANDLE, PWSTR};
use windows::Win32::Media::Multimedia;
use windows::Win32::System::Threading;
use windows::Win32::System::WindowsProgramming::INFINITE;

// can't find this constant in the library, taken from the online win32 documentation
// https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights
const TIMER_ALL_ACCESS: u32 = 0x1F0003;

// https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject
const WAIT_FAILED: u32 = 0xFFFFFFFF;

lazy_static! {
    static ref WAITABLE_TIMER: HANDLE = {
        unsafe {
            // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateWaitableTimerW.html
            // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createwaitabletimerw
            // Threading::CreateWaitableTimerW(ptr::null(), true, PWSTR::default())

            // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateWaitableTimerExW.html
            // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createwaitabletimerexw 
            Threading::CreateWaitableTimerExW(ptr::null(), PWSTR::default(), Threading::CREATE_WAITABLE_TIMER_HIGH_RESOLUTION, TIMER_ALL_ACCESS)
        }
    };
}

// positive value = absolute time / timestamp 
// negative value = relative time
// timeunit = 100 ns
fn duration_to_wait_time(dur: Duration) -> i64 {
    if dur.is_zero() {
        return -1;
    }
    let ns = dur.as_nanos();
    if ns > std::i64::MAX as u128 {
        return std::i64::MIN / 100;
    } else {
        return -(ns as i64) / 100;
    }
}

pub fn get_timecaps() -> (u32, u32) {
    let mut timecaps: Multimedia::TIMECAPS = Default::default();
    unsafe {
        // Windows Docs:
        // https://docs.microsoft.com/en-us/windows/win32/api/timeapi/ns-timeapi-timecaps
        // https://docs.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timegetdevcaps
        Multimedia::timeGetDevCaps(&mut timecaps, mem::size_of::<Multimedia::TIMECAPS>() as u32);
    }
    return (timecaps.wPeriodMin, timecaps.wPeriodMax);
}

pub fn sleep(dur: Duration) {
    let timer: HANDLE = *WAITABLE_TIMER;
    let wait_time = duration_to_wait_time(dur);
    unsafe {
        // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.SetWaitableTimer.html
        // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-setwaitabletimer
        if !Threading::SetWaitableTimer(timer, &wait_time, 0, None, ptr::null(), false).as_bool() {
            panic!("SetWaitableTimer failed");
        }
        // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.WaitForSingleObject.html
        // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject
        if WAIT_FAILED == Threading::WaitForSingleObject(timer, INFINITE) {
            panic!("WaitForSingleObject failed");
        }
    }
}
