use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
}

const QUEUE_SIZE: usize = 128;
static mut KEY_QUEUE: [Option<char>; QUEUE_SIZE] = [None; QUEUE_SIZE];
static mut QUEUE_HEAD: usize = 0;
static mut QUEUE_TAIL: usize = 0;

pub fn poll() {
    let mut port = Port::new(0x60);
    let mut status_port = Port::new(0x64);

    // Check if data is available (Bit 0 of Status Register)
    let status: u8 = unsafe { status_port.read() };
    if (status & 1) != 0 {
        let scancode: u8 = unsafe { port.read() };
        add_scancode(scancode);
    }
}

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => push_key(character),
                DecodedKey::RawKey(_) => {},
            }
        }
    }
}

fn push_key(c: char) {
    unsafe {
        let next = (QUEUE_HEAD + 1) % QUEUE_SIZE;
        if next != QUEUE_TAIL {
            KEY_QUEUE[QUEUE_HEAD] = Some(c);
            QUEUE_HEAD = next;
        }
    }
}

pub fn pop_key() -> Option<char> {
    unsafe {
        if QUEUE_HEAD == QUEUE_TAIL {
            None
        } else {
            let c = KEY_QUEUE[QUEUE_TAIL];
            QUEUE_TAIL = (QUEUE_TAIL + 1) % QUEUE_SIZE;
            c
        }
    }
}
