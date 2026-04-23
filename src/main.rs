use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

#[repr(C)]
struct INPUT {
    r#type: u32,
    mi: MOUSEINPUT,
}

#[repr(C)]
struct MOUSEINPUT {
    dx: i32,
    dy: i32,
    mouse_data: u32,
    dw_flags: u32,
    time: u32,
    dw_extra_info: usize,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn SendInput(cInputs: u32, pInputs: *mut INPUT, cbSize: i32) -> u32;
    fn GetAsyncKeyState(vKey: i32) -> i16;
}

const INPUT_MOUSE: u32 = 0;
const MOUSEEVENTF_LEFTDOWN: u32 = 0x0002;
const MOUSEEVENTF_LEFTUP: u32 = 0x0004;

const VK_F6: i32 = 0x75;
const VK_F7: i32 = 0x76;

fn send(flags: u32) {
    unsafe {
        let mut input = INPUT {
            r#type: INPUT_MOUSE,
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouse_data: 0,
                dw_flags: flags,
                time: 0,
                dw_extra_info: 0,
            },
        };

        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

fn click() {
    send(MOUSEEVENTF_LEFTDOWN);
    thread::sleep(Duration::from_millis(2));
    send(MOUSEEVENTF_LEFTUP);
}

fn main() {
    let running = Arc::new(AtomicBool::new(false));
    let r = running.clone();

    thread::spawn(move || {
        loop {
            if r.load(Ordering::Relaxed) {
                click();
                thread::sleep(Duration::from_millis(50)); // speed (lower = faster)
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        }
    });

    println!("F6 = start | F7 = stop");

    loop {
        unsafe {
            if GetAsyncKeyState(VK_F6) & 1 != 0 {
                running.store(true, Ordering::Relaxed);
                println!("Started");
            }

            if GetAsyncKeyState(VK_F7) & 1 != 0 {
                running.store(false, Ordering::Relaxed);
                println!("Stopped");
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}