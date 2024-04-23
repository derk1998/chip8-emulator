use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use std::{fs::File, io::stdout};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crossterm::{cursor, event, queue, style, terminal, ExecutableCommand};

mod display;

use display::Display;

mod processor;

use processor::{Chip8, Key, Keypad, Memory};

fn load_font(memory: &mut [u8]) {
    let font = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F,
    ];

    memory[0x50..(0x50 + font.len())].clone_from_slice(&font);
}

fn read_rom(file_path: &str, memory: &mut [u8]) -> Result<Vec<u8>, io::Error> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut file_buffer = vec![];
    match file.read_to_end(&mut file_buffer) {
        Ok(_) => return Ok(file_buffer),
        Err(e) => return Err(e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: [u8; 4096] = [0; 4096];
    load_font(&mut memory);
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return Err("Expected filename as argument".into());
    }

    let rom = read_rom(&args[1], &mut memory)?;

    memory[0x200..(0x200 + rom.len())].clone_from_slice(&rom);

    let mut display = Display::new(64, 32);

    let mut chip8 = Chip8::new(&mut display, Memory { data: memory });

    loop {
        let event_available = event::poll(Duration::from_millis(1));

        if event_available.unwrap() {
            match event::read().unwrap() {
                event::Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    break;
                }
                event::Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind,
                    ..
                }) => match kind {
                    KeyEventKind::Press => match c {
                        '1' => chip8.handle_key_down(Key::Key1),
                        '2' => chip8.handle_key_down(Key::Key2),
                        '3' => chip8.handle_key_down(Key::Key3),
                        '4' => chip8.handle_key_down(Key::KeyC),
                        'q' => chip8.handle_key_down(Key::Key4),
                        'w' => chip8.handle_key_down(Key::Key5),
                        'e' => chip8.handle_key_down(Key::Key6),
                        'r' => chip8.handle_key_down(Key::KeyD),
                        'a' => chip8.handle_key_down(Key::Key7),
                        's' => chip8.handle_key_down(Key::Key8),
                        'd' => chip8.handle_key_down(Key::Key9),
                        'f' => chip8.handle_key_down(Key::KeyD),
                        'z' => chip8.handle_key_down(Key::KeyA),
                        'x' => chip8.handle_key_down(Key::Key0),
                        'c' => chip8.handle_key_down(Key::KeyB),
                        'v' => chip8.handle_key_down(Key::KeyF),
                        _ => {}
                    },
                    KeyEventKind::Release => match c {
                        '1' => chip8.handle_key_up(Key::Key1),
                        '2' => chip8.handle_key_up(Key::Key2),
                        '3' => chip8.handle_key_up(Key::Key3),
                        '4' => chip8.handle_key_up(Key::KeyC),
                        'q' => chip8.handle_key_up(Key::Key4),
                        'w' => chip8.handle_key_up(Key::Key5),
                        'e' => chip8.handle_key_up(Key::Key6),
                        'r' => chip8.handle_key_up(Key::KeyD),
                        'a' => chip8.handle_key_up(Key::Key7),
                        's' => chip8.handle_key_up(Key::Key8),
                        'd' => chip8.handle_key_up(Key::Key9),
                        'f' => chip8.handle_key_up(Key::KeyE),
                        'z' => chip8.handle_key_up(Key::KeyA),
                        'x' => chip8.handle_key_up(Key::Key0),
                        'c' => chip8.handle_key_up(Key::KeyB),
                        'v' => chip8.handle_key_up(Key::KeyF),
                        _ => {}
                    },
                    _ => {}
                },
                _ => (),
            }
        }

        chip8.emulate_cycle();
        thread::sleep(Duration::from_millis(3));
    }
    Ok(())
}
