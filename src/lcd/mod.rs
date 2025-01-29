use rppal::gpio::{Gpio, OutputPin, Level};
use std::{thread, time::Duration};


/* TC1602A-01T */
// https://cdn-shop.adafruit.com/datasheets/TC1602A-01T.pdf
// +---------+---------+--------+----------------------------------------------------------------------------+
// | PinNo.  | Symbol  | Level  |                                Description                                 |
// +---------+---------+--------+----------------------------------------------------------------------------+
// | 1       | VSS     | 0V     | Ground.                                                                    |
// | 2       | VDD     | +5.0V  | Power supply for logic operating.                                          |
// | 3       | V0      | --     | Adjusting supply voltage for LCD driving.                                  |
// | 4       | RS      | H/L    | A signal for selecting registers:                                          |
// | |       |         |        | 1. Data Register (for read and write)                                      |
// | |       |         |        | 2. Instruction Register (for write), Busy flag-Address Counter (for read). |
// | 5       | R/W     | H/L    | R/W = “H”: Read mode. R/W = “L”: Write mode.                               |
// | 6       | E       | H/L    | An enable signal for writing or reading data.                              |
// | 7       | DB0     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 8       | DB1     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 9       | DB2     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 10      | DB3     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 11      | DB4     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 12      | DB5     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 13      | DB6     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 14      | DB7     | H/L    | This is an 8-bit bi-directional data bus.                                  |
// | 15      | LED+    | +5.0V  | Power supply for backlight.                                                |
// | 16      | LED-    | 0V     | The backlight ground                                                       |
// +---------+---------+--------+----------------------------------------------------------------------------+



pub mod pins {
    pub const RS: u8 = 4;
    pub const E: u8 = 17;
    pub const DB4: u8 = 18;
    pub const DB5: u8 = 22;
    pub const DB6: u8 = 23;
    pub const DB7: u8 = 24;
}

pub mod lines {
    pub const LINE_1: u8 = 0x80; // 0b10000000
    pub const LINE_2: u8 = 0xC0; // 0b11000000
}

pub mod instructions {
    pub const CLEAR_DISPLAY: u8 = 0x01; // (binary: 0000 0001) Clear display
    pub const ENTRY_MODE_SET: u8 = 0x06; // (binary: 0000 0110) Increment cursor, no display shift
    pub const DISPLAY_ON: u8 = 0x0C; // (binary: 0000 1100) Display ON, Cursor OFF, Blink OFF
    pub const FUNCTION_SET: u8 = 0x28; // (binary: 0010 1000) 4-bit mode, 2 lines, 5x8 font
    pub const SHIFT_RIGHT: u8 = 0x14; // (binary: 0001 0100) Shift display right
}


// A helper structure to manage the LCD
pub struct Lcd {
    rs: OutputPin,
    e: OutputPin,
    db4: OutputPin,
    db5: OutputPin,
    db6: OutputPin,
    db7: OutputPin,
}



impl Lcd {
    pub fn new() -> Self {
        let gpio = Gpio::new().expect("Failed to access GPIO");
        Self {
            rs: gpio.get(pins::RS).map_err(|e| {
                println!("Failed to access RS pin: {:?}", e);
            }).expect("Failed to access RS pin").into_output(),
            e: gpio.get(pins::E).expect("Failed to access E pin").into_output(),
            db4: gpio.get(pins::DB4).expect("Failed to access DB4 pin").into_output(),
            db5: gpio.get(pins::DB5).expect("Failed to access DB5 pin").into_output(),
            db6: gpio.get(pins::DB6).expect("Failed to access DB6 pin").into_output(),
            db7: gpio.get(pins::DB7).expect("Failed to access DB7 pin").into_output(),
        }
    }

    // Pulse the enable pin
    fn pulse_enable(&mut self) {
        self.e.set_high();
        thread::sleep(Duration::from_micros(1));
        self.e.set_low();
        thread::sleep(Duration::from_micros(50));
    }

    fn clear_buses(&mut self) {
        self.db4.write(Level::Low);
        self.db5.write(Level::Low);
        self.db6.write(Level::Low);
        self.db7.write(Level::Low);
    }

    // Send a nibble (1/2 byte or 4 bits) to the LCD
    fn send_nibble(&mut self, nibble: u8) {
        self.clear_buses();
        self.db4.write(if (nibble & 0b0001) != 0 { Level::High } else { Level::Low });
        self.db5.write(if (nibble & 0b0010) != 0 { Level::High } else { Level::Low });
        self.db6.write(if (nibble & 0b0100) != 0 { Level::High } else { Level::Low });
        self.db7.write(if (nibble & 0b1000) != 0 { Level::High } else { Level::Low });
        self.pulse_enable();
    }

    // Send a byte (1 byte or 8 bits) to the LCD (two nibbles)
    fn send_byte(&mut self, byte: u8, is_data: bool) {
        self.rs.write(if is_data {Level::High} else {Level::Low}); // Set RS: 0 for command, 1 for data
        self.send_nibble(byte >> 4); // Send high nibble
        self.send_nibble(byte & 0x0F); // Send low nibble
        thread::sleep(Duration::from_micros(50));
    }

    // Set functions (data length, number of lines, font size)
    pub fn set_function(&mut self, data_length: u8, num_lines: u8, font: u8) {
        let function = 0b00100000 | (if data_length == 8 { 0b00010000 } else { 0 }) | (if num_lines == 2 { 0b00001000 } else { 0 }) | (if font == 5 { 0b00000100 } else { 0 });
        self.send_byte(function, false);
    }

    // Set display mode (display on, cursor on, cursor blink)
    pub fn set_display_mode(&mut self, display_on: bool, cursor_on: bool, cursor_blink: bool) {
        let display_mode = 0b00001000 | (if display_on { 0b00000100 } else { 0 }) | (if cursor_on { 0b00000010 } else { 0 }) | (if cursor_blink { 0b00000001 } else { 0 });
        self.send_byte(display_mode, false);
    }

    // Move to a specific line
    pub fn move_to_line(&mut self, line: u8) {
        let mut line_bytes = lines::LINE_1;
        if line == 2 {
            line_bytes = lines::LINE_2;
        }
        self.send_byte(line_bytes, false);
    }

    // Shift right
    pub fn shift_right(&mut self) {
        self.send_byte(instructions::SHIFT_RIGHT, false);
    }

    // Clear display
    pub fn clear_display(&mut self) {
        self.send_byte(instructions::CLEAR_DISPLAY, false);
        thread::sleep(Duration::from_millis(2));
    }

    // Initialize the LCD
    pub fn initialize(&mut self) {
        thread::sleep(Duration::from_millis(15)); // Wait for LCD to power on

        self.send_nibble(0x03); // Set 8-bit mode
        thread::sleep(Duration::from_millis(5));
        self.send_nibble(0x03); // Set 8-bit mode
        thread::sleep(Duration::from_micros(150));
        self.send_nibble(0x03); // Set 8-bit mode
        self.send_nibble(0x02); // Set 4-bit mode

        // Set function, display, clear, and entry mode
        self.set_function(4, 2, 10);
        self.set_display_mode(true, true, true); // Display ON, cursor ON, blink ON
        self.clear_display();
        self.send_byte(instructions::ENTRY_MODE_SET, false); // Set entry mode*/
    }

    // Display a message
    pub fn display_message(&mut self, message: &str) {
        for ch in message.chars() {
            self.send_byte(ch as u8, true); // Send each character as data
        }
    }
}
