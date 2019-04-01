extern crate lazy_static;
extern crate spin;
extern crate volatile;


use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::vga_buffer::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//sets up the buffer size
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//this creates the buffer in a 2-d array
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//this creates a MOVER object
pub struct Mover {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    char_moved: u8,
    hidden_char_col_pos: usize,
    hidden_char_row_pos: usize,
}

impl Mover {
    pub fn write_byte(&mut self, byte: u8) {


            match byte {
                b'w' => {
                    if self.row_position > 0 {
                        let x = self.buffer.chars[self.row_position - 1][self.column_position].read().color_code;
                        if x != ColorCode::new(Color::Yellow, Color::Black){
                            self.delete_char();
                            self.row_position -= 1;
                            self.place_moving_char();
                        }
                    }else {
                        self.delete_char();
                        if self.row_position <= 0{
                            self.row_position = BUFFER_HEIGHT;
                        }
                        self.row_position -= 1;
                        self.place_moving_char();
                    }




                }
                b's' => {
                    if self.row_position < BUFFER_HEIGHT - 1{
                        let x = self.buffer.chars[self.row_position + 1][self.column_position].read().color_code;
                        if x != ColorCode::new(Color::Yellow, Color::Black){
                            self.delete_char();
                            self.row_position += 1;

                            self.place_moving_char();
                        }
                    }else {
                        self.delete_char();
                        if self.row_position >= BUFFER_HEIGHT - 1 {
                                self.row_position = 0;
                        }
                        self.place_moving_char();
                    }




                }
                b'd' => {
                    if self.column_position < BUFFER_WIDTH - 1 {
                        let x = self.buffer.chars[self.row_position][self.column_position + 1].read().color_code;
                        if x != ColorCode::new(Color::Yellow, Color::Black) {
                            self.delete_char();

                            self.column_position += 1;


                            self.place_moving_char();
                        }
                    }else {
                        self.delete_char();
                        if self.column_position >= BUFFER_WIDTH - 1 {
                            self.column_position = 0;
                        }
                        self.place_moving_char();
                    }

                }
                b'a' => {
                    if self.column_position > 0 {
                        let x = self.buffer.chars[self.row_position][self.column_position - 1].read().color_code;
                        if x != ColorCode::new(Color::Yellow, Color::Black) {
                            self.delete_char();
                            self.column_position -= 1;
                            self.place_moving_char();
                        }
                    }else{
                        self.delete_char();
                        if self.column_position <= 0 {
                            self.column_position = BUFFER_WIDTH - 1;
                        }
                        self.place_moving_char();
                    }

                }
                byte=> {
                    self.char_moved = byte;
                    self.place_char();
                }



        }
    }

    pub fn place_moving_char(&mut self){

        let color_code = self.color_code;

        let row = self.row_position;
        let column = self.column_position;

        if row == self.hidden_char_row_pos && column == self.hidden_char_col_pos{
            self.buffer.chars[row][column].write( ScreenChar {
                ascii_character: b'+',
                color_code
            });
        }else{
            self.buffer.chars[row][column].write( ScreenChar {
                ascii_character: self.char_moved,
                color_code
            });
        }

        let x = self.buffer.chars[row][column].read().color_code;
//        if x == ColorCode::new(Color::Yellow, Color::Black){
//
//        }else{
//            self.buffer.chars[row][column].write( ScreenChar {
//                ascii_character: self.char_moved,
//                color_code
//            });
//        }

    }

    pub fn place_char(&mut self){
        let color_code = self.color_code;

        let row = self.row_position;
        let column = self.column_position;


        self.buffer.chars[row][column].write( ScreenChar {
            ascii_character: self.char_moved,
            color_code
        });
    }



    pub fn delete_char(&mut self){
        let row = self.row_position;
        let column = self.column_position;
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black),
        };
        self.buffer.chars[row][column].write(blank);
    }


    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

}


//this says create the move when _move gets called
lazy_static! {
    pub static ref MOVER: Mutex<Mover> = Mutex::new(Mover {
        column_position: 40,
        row_position: 11,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        char_moved: b'0',
        hidden_char_col_pos: 2,
        hidden_char_row_pos: 12,
    });
}

impl fmt::Write for Mover {
    //this will actually display the character
    fn write_str(&mut self, s: &str) -> fmt::Result {

        self.write_string(s);
        Ok(())
    }
}



#[doc(hidden)]
pub fn _move(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        MOVER.lock().write_fmt(args).unwrap();
    });
}