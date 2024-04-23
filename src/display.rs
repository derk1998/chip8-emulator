use std::io::{stdout, Write};

use crossterm::{
    cursor, queue,
    style::{self, Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand,
};

pub struct Display {
    width: u16,
    height: u16,
    surface: Vec<u8>,
}

impl Display {
    pub fn new(width: u16, height: u16) -> Display {
        stdout()
            .execute(terminal::EnterAlternateScreen)
            .expect("Could not enter alternate buffer");

        Display {
            width,
            height,
            surface: vec![0_u8; (width * height).into()],
        }
    }

    pub fn clear(&mut self) {
        self.surface.fill(0);
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) -> bool {
        if self.surface[x + y * self.width as usize] == 1 {
            self.surface[x + y * self.width as usize] = 0;
            return false;
        }

        self.surface[x + y * self.width as usize] = 1;

        true
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16) {
        if self.surface[(x + y * self.width) as usize] == 1 {
            queue!(stdout(), SetForegroundColor(Color::White))
                .expect("Could not write to the buffer");
        } else {
            queue!(stdout(), SetForegroundColor(Color::Black))
                .expect("Could not write to the buffer");
        }

        if y < self.height - 1 && self.surface[(x + (y + 1) * self.width) as usize] == 1 {
            queue!(stdout(), SetBackgroundColor(Color::White))
                .expect("Could not write to the buffer");
        } else {
            queue!(stdout(), SetBackgroundColor(Color::Black))
                .expect("Could not write to the buffer");
        }

        let y_coordinate = ((y as f32) / 2.0).floor() as u16;
        let x_coordinate = x;

        queue!(
            stdout(),
            cursor::MoveTo(x_coordinate, y_coordinate),
            cursor::Hide,
            style::Print("▀"),
            ResetColor
        )
        .expect("Could not write to the buffer");
    }

    pub fn display(&mut self) {
        for x in 0..self.width {
            for y in (0..self.height).step_by(2) {
                self.draw_pixel(x, y);
            }
        }
        stdout().flush().expect("Could not flush stdout");
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        stdout()
            .execute(terminal::LeaveAlternateScreen)
            .expect("Could not leave the alternate buffer");
    }
}
