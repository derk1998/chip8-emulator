use std::io::Read;
use std::time::Duration;
use std::{fs::File, io::stdout};

use crossterm::{cursor, event, queue, style, terminal, ExecutableCommand};

mod display;

use display::Display;

mod processor;

use processor::{Chip8, Memory};

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

fn main() {
    let mut stdout = stdout();
    stdout
        .execute(terminal::EnterAlternateScreen)
        .expect("Could not enter alternate buffer");

    let width: u16 = 64;
    let height: u16 = 32;

    let mut memory: [u8; 4096] = [0; 4096];
    load_font(&mut memory);

    let mut file = File::open("test_opcode.ch8").expect("no such file found");
    let mut file_buffer = vec![];
    let res = file.read_to_end(&mut file_buffer);
    memory[0x200..(0x200 + file_buffer.len())].clone_from_slice(&file_buffer);

    //First try with 1 pix = 8 bit
    let mut buffer = vec![0_u8; (width * height).into()];

    //Write a pixel to test
    buffer[usize::from(width)] = 1;

    let mut display = Display::new(&mut stdout, width, height);
    let mut chip8 = Chip8::new(&mut display, Memory { data: memory });
    // display.display(buffer.as_slice());

    buffer[usize::from(width * 30)] = 1;

    //For now this will work, probably have to make some windowing system
    // stdout
    //     .execute(cursor::MoveToNextLine(2))
    //     .expect("Could not move to next line");

    //Our ram

    // load_font(&mut memory);
    // memory[0x99] = 0x00;
    // memory[0x9A] = 0xE0;

    // //Program counter
    // let mut program_counter: u16 = 0x99;

    // //Index register I
    // let mut index_register_i: u16 = 0;

    // //Stack
    // let mut stack: Vec<u16> = Vec::new();

    // //Timers
    // let delay_timer: u8 = 0;
    // let sound_timer: u8 = 0;

    //Keypad
    let keypad = [
        ['1', '2', '3', 'C'],
        ['4', '5', '6', 'D'],
        ['7', '8', '9', 'E'],
        ['A', '0', 'B', 'F'],
    ];

    // for i in 0..4 {
    //     for j in 0..4 {
    //         queue!(stdout, style::Print(keypad[i][j]), style::Print(' '))
    //             .expect("Could not write to the buffer");
    //     }
    //     stdout
    //         .execute(cursor::MoveToNextLine(1))
    //         .expect("Could not move to next line");
    // }
    // stdout
    //     .execute(cursor::MoveToNextLine(1))
    //     .expect("Could not move to next line");

    // stdout
    //     .execute(cursor::Hide)
    //     .expect("Could not hide the cursor");
    loop {
        let event_available = event::poll(Duration::from_millis(1000 / 700));

        if event_available.unwrap() {
            match event::read().unwrap() {
                event::Event::Key(_) => {
                    stdout
                        .execute(terminal::LeaveAlternateScreen)
                        .expect("Could not leave the alternate buffer");
                    break;
                }
                _ => (),
            }
        }

        chip8.emulate_cycle();
    }
}
