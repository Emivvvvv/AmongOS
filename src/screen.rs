use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::screen::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0x0, White = 0xF,
    Blue = 0x1, BrightBlue = 0x9,
    Green = 0x2, BrightGreen = 0xA,
    Cyan = 0x3, BrightCyan = 0xB,
    Red = 0x4, BrightRed = 0xC,
    Magenta = 0x5, BrightMagenta = 0xD,
    Brown = 0x6, Yellow = 0xE,
    Gray = 0x7, DarkGray = 0x8,
}

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

#[repr(transparent)]
#[derive(Debug)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.delete_among_os();
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        self.draw_among_os();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    // fn time_tick(&mut self) {
    //     let mut screen_char_1 = self.buffer.chars[0][BUFFER_WIDTH - 2].read();
    //     let mut screen_char_2 = self.buffer.chars[0][BUFFER_WIDTH - 1].read();
    //     screen_char_1.color_code = ColorCode::new(Color::Yellow, Color::Red);
    //     screen_char_2.color_code = ColorCode::new(Color::Yellow, Color::Red);
    //     self.buffer.chars[0][BUFFER_WIDTH - 2].write(screen_char_1);
    //     self.buffer.chars[0][BUFFER_WIDTH - 1].write(screen_char_2);
    //
    //     //a delay function is needed
    //
    //     screen_char_1.color_code = ColorCode::new(Color::Yellow, Color::White);
    //     screen_char_2.color_code = ColorCode::new(Color::Yellow, Color::White);
    //     self.buffer.chars[0][BUFFER_WIDTH - 2].write(screen_char_1);
    //     self.buffer.chars[0][BUFFER_WIDTH - 1].write(screen_char_2);
    // }

    fn draw_among_os(&mut self) {
        //  [x, x, x, x, x, x, y, c]
        //  x => x(column) coordinate slots. 0s will be excluded.
        //  y => y(row) coordinate slot,
        //  c => Color code of the red imposter from among us's part
        let among_os: [[isize; 8]; 9] = [
            [38, 39, 40, -1, -1, -1, 9, 0x4],
            [37, 38, 39, 40, 41, -1, 10, 0x4],
            [36, 37, 38, -1, -1, -1, 11, 0x4],
            [36, 37, 38, -1, -1, -1, 12, 0x4],
            [36, 37, 38, 39, 40, 41, 13, 0x4],
            [36, 37, 38, 39, 40, 41, 14, 0x4],
            [37, 38, 40, 41, -1, -1, 15, 0x4],
            [39, 40, 41, 42, -1, -1, 11, 0xB],
            [39, 40, 41, 42, -1, -1, 12, 0xB]];

        among_os.into_iter().for_each(|pack| {
            self.draw_line_among_os(&pack[..6], pack[6], pack[7])
        });
    }

    fn draw_line_among_os(&mut self, x_list: &[isize], row: isize, color: isize) {
        let color_code = if color == 0x4 {ColorCode::new(Color::Yellow, Color::Red)}
        else {ColorCode::new(Color::Yellow, Color::BrightCyan)};

        self.draw_line(x_list, row, color_code);
    }

    fn delete_among_os(&mut self) {
        for row in 9..=15 {
            for col in 0..BUFFER_WIDTH {
                let mut screen_char = self.buffer.chars[row][col].read();
                screen_char.color_code = ColorCode::new(Color::Yellow, Color::Black);
                self.buffer.chars[row][col].write(screen_char);
            }
        }
    }

    // -1 values in x_list will be omitted.
    fn draw_line(&mut self, x_list: &[isize], row: isize, color_code: ColorCode) {
        for col in x_list {
            if *col == -1 { continue }
            let mut screen_char = self.buffer.chars[row as usize][*col as usize].read();
            screen_char.color_code = color_code;
            self.buffer.chars[row as usize][*col as usize].write(screen_char);
        }
    }
}

pub fn welcome() {
    println!("AMONG OS {}", VERSION);
    println!("Created by Emirhan Tala");
    println!();
    WRITER.lock().draw_among_os();
}

// pub fn tick() {
//     WRITER.lock().time_tick();
// }

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
