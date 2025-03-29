TC1602A-01T Driver for Rust
===========================
## This is an example usage of the TC1602A LCD display

- The core of the lcd code sits in
```/src/lcd/mod.rs```
- This example displays the current BTC price so the api functionality for that is in
```/src/api/mod.rs```

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
