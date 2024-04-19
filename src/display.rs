use std::{io::Write, os::windows};

use crossterm::{
    cursor, queue,
    style::{self, Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub struct Display<'a> {
    stdout: &'a std::io::Stdout,
    width: u16,
    height: u16,
    surface: Vec<u8>,
}

impl Display<'_> {
    pub fn new(stdout: &std::io::Stdout, width: u16, height: u16) -> Display {
        Display {
            stdout,
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

    pub fn display(&mut self) {
        for x in 0..self.width {
            for y in (0..self.height).step_by(2) {
                if self.surface[(x + y * self.width) as usize] == 1 {
                    queue!(self.stdout, SetForegroundColor(Color::White))
                        .expect("Could not write to the buffer");
                } else {
                    queue!(self.stdout, SetForegroundColor(Color::Black))
                        .expect("Could not write to the buffer");
                }

                if y < self.height - 1 && self.surface[(x + (y + 1) * self.width) as usize] == 1 {
                    queue!(self.stdout, SetBackgroundColor(Color::White))
                        .expect("Could not write to the buffer");
                } else {
                    queue!(self.stdout, SetBackgroundColor(Color::Black))
                        .expect("Could not write to the buffer");
                }

                let y_coordinate = ((y as f32) / 2.0).floor() as u16;
                let x_coordinate = x;

                queue!(
                    self.stdout,
                    cursor::MoveTo(x_coordinate, y_coordinate),
                    style::Print("▀"),
                    ResetColor
                )
                .expect("Could not write to the buffer");
            }
        }
        self.stdout.flush().expect("Could not flush stdout");
    }
}
