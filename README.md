TC1602A-01T Driver for Rust
===========================
## This is an example usage of the TC1602A LCD display

- The core of the lcd code sits in
```/src/lcd/mod.rs```
- This example displays the current BTC price so the api functionality for that is in
```/src/api/mod.rs```

### LCD Functions
| Function           | Parameters                        | Description                                              |
|--------------------|----------------------------------|----------------------------------------------------------|
| `new()`           | None                             | Creates a new instance of the LCD.                      |
| `pulse_enable()`  | None                             | Sends a short pulse to the enable pin.                  |
| `clear_buses()`   | None                             | Sets all data bus pins to LOW.                          |
| `send_nibble()`   | `nibble: u8`                     | Sends 4 bits to the LCD.                                |
| `send_byte()`     | `byte: u8, is_data: bool`        | Sends 8 bits (two nibbles) to the LCD.                  |
| `set_function()`  | `data_length: u8, num_lines: u8, font: u8` | Configures the display mode (4-bit/8-bit, lines, font). |
| `set_display_mode()` | `display_on: bool, cursor_on: bool, cursor_blink: bool` | Configures display settings. |
| `move_to_line()`  | `line: u8`                      | Moves the cursor to a specific line.                    |
| `shift_right()`   | None                             | Shifts the display to the right.                        |
| `clear_display()` | None                             | Clears the LCD display.                                 |
| `initialize()`    | None                             | Initializes the LCD with default settings.              |
| `display_message()` | `message: &str`               | Displays a string on the LCD.                           |

Data sheet for the LCD:
| PinNo. | Symbol | Level  | Description |
|--------|--------|--------|-------------|
| 1      | VSS    | 0V     | Ground. |
| 2      | VDD    | +5.0V  | Power supply for logic operating. |
| 3      | V0     | --     | Adjusting supply voltage for LCD driving. |
| 4      | RS     | H/L    | A signal for selecting registers:<br>1. Data Register (for read and write)<br>2. Instruction Register (for write), Busy flag-Address Counter (for read). |
| 5      | R/W    | H/L    | R/W = “H”: Read mode.<br>R/W = “L”: Write mode. |
| 6      | E      | H/L    | An enable signal for writing or reading data. |
| 7      | DB0    | H/L    | This is an 8-bit bi-directional data bus. |
| 8      | DB1    | H/L    | This is an 8-bit bi-directional data bus. |
| 9      | DB2    | H/L    | This is an 8-bit bi-directional data bus. |
| 10     | DB3    | H/L    | This is an 8-bit bi-directional data bus. |
| 11     | DB4    | H/L    | This is an 8-bit bi-directional data bus. |
| 12     | DB5    | H/L    | This is an 8-bit bi-directional data bus. |
| 13     | DB6    | H/L    | This is an 8-bit bi-directional data bus. |
| 14     | DB7    | H/L    | This is an 8-bit bi-directional data bus. |
| 15     | LED+   | +5.0V  | Power supply for backlight. |
| 16     | LED-   | 0V     | The backlight ground. |

### LCD Instructions
| Instruction         | Hex Code | Description                                          |
|---------------------|---------|------------------------------------------------------|
| CLEAR_DISPLAY      | 0x01    | Clears the display.                                  |
| ENTRY_MODE_SET     | 0x06    | Cursor moves right, no display shift.               |
| DISPLAY_ON         | 0x0C    | Display ON, Cursor OFF, Blink OFF.                  |
| FUNCTION_SET       | 0x28    | 4-bit mode, 2 lines, 5x8 font.                      |
| SHIFT_RIGHT        | 0x14    | Shifts display to the right.                        |

### Line Addresses
| Line   | Hex Code | Description               |
|--------|---------|---------------------------|
| LINE_1 | 0x80    | Start address of line 1.  |
| LINE_2 | 0xC0    | Start address of line 2.  |

