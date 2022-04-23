use std::mem;
use std::{ptr, time::Duration};

use lazy_static::lazy_static;

use windows::runtime::Handle;
use windows::Win32::Foundation::{CloseHandle, HANDLE, PWSTR};
use windows::Win32::Media::Multimedia;
use windows::Win32::System::Threading;
use windows::Win32::System::WindowsProgramming::INFINITE;

// can't find this constant in the library, taken from the online win32 documentation
// https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights
const TIMER_ALL_ACCESS: u32 = 0x1F0003;

lazy_static! {
    // check if high resolution timers are available and store the pointer to the best available implementation.
    static ref SLEEP_FN: fn(Duration) = {
        unsafe {
            let timer = create_waitable_high_resolution_timer();
            if timer.is_invalid() {
                sleep_fallback
            } else {
                CloseHandle(timer);
                sleep_high_resolution
            }
        }
    };
}

pub fn get_timecaps() -> (u32, u32) {
    let mut timecaps: Multimedia::TIMECAPS = Default::default();
    unsafe {
        // Windows Docs:
        // https://docs.microsoft.com/en-us/windows/win32/api/timeapi/ns-timeapi-timecaps
        // https://docs.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timegetdevcaps
        Multimedia::timeGetDevCaps(&mut timecaps, mem::size_of::<Multimedia::TIMECAPS>() as u32);
    }
    (timecaps.wPeriodMin, timecaps.wPeriodMax)
}

pub fn sleep(dur: Duration) {
    if dur.is_zero() {
        unsafe { Threading::Sleep(0) }
    } else {
        (*SLEEP_FN)(dur)
    }
}

// Sleep implementation based on CreateWaitableTimerExW with the flag CREATE_WAITABLE_TIMER_HIGH_RESOLUTION.
fn sleep_high_resolution(dur: Duration) {
    let wait_time = duration_to_wait_time(dur);
    unsafe {
        let timer = create_waitable_high_resolution_timer();
        if timer.is_invalid() {
            panic!("CreateWaitableTimerExW failed")
        }
        // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.SetWaitableTimer.html
        // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-setwaitabletimer
        if !Threading::SetWaitableTimer(timer, &wait_time, 0, None, ptr::null(), false).as_bool() {
            CloseHandle(timer);
            panic!("SetWaitableTimer failed");
        }
        // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.WaitForSingleObject.html
        // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject
        if Threading::WaitForSingleObject(timer, INFINITE) != Threading::WAIT_OBJECT_0 {
            CloseHandle(timer);
            panic!("WaitForSingleObject failed");
        }
        CloseHandle(timer);
    }
}

// Sleep implementation based on the default Win32 Sleep API.
fn sleep_fallback(dur: Duration) {
    let milliseconds = std::cmp::min(dur.as_millis(), INFINITE as u128) as u32;
    unsafe { Threading::Sleep(milliseconds) }
}

unsafe fn create_waitable_high_resolution_timer() -> HANDLE {
    // Rust: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateWaitableTimerExW.html
    // Windows: https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createwaitabletimerexw
    Threading::CreateWaitableTimerExW(
        ptr::null(),
        PWSTR::default(),
        Threading::CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
        TIMER_ALL_ACCESS,
    )
}

// Converts the duration into the format expected by SetWaitableTimer.
// positive value = absolute time / timestamp
// negative value = relative time
// timeunit = 100 ns
fn duration_to_wait_time(dur: Duration) -> i64 {
    let duration_in_100_ns = dur.as_nanos() / 100;
    i64::try_from(duration_in_100_ns).map(|x| -x).unwrap_or(std::i64::MIN)
}
