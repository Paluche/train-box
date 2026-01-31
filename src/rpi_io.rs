//! GPIO pin-out
//!
//! Careful to not mix-up pin numbers and GPIO numbers.
//!
//!                  SD card side
//!                  ┌────┬────┐
//!        3v3 power │ 01 │  2 │ 5V power
//!                  ├────┼────┤
//!     GPIO 2 (SDA) │ 03 │  4 │ 5V power
//!                  ├────┼────┤
//!     GPIO 3 (SCL) │ 05 │  6 │ Ground
//!                  ├────┼────┤
//!  GPIO 4 (GPCLK0) │ 07 │  8 │ GPIO 14 (TXD)
//!                  ├────┼────┤
//!           Ground │ 09 │ 10 │ GPIO 15 (RXD)
//!                  ├────┼────┤
//!          GPIO 17 │ 11 │ 12 │ GPIO 18 (PCM_CLK)
//!                  ├────┼────┤
//!          GPIO 27 │ 13 │ 14 │ Ground
//!                  ├────┼────┤
//!          GPIO 22 │ 15 │ 16 │ GPIO 23
//!                  ├────┼────┤
//!        3v3 power │ 17 │ 18 │ GPIO 24
//!                  ├────┼────┤
//!   GPIO 10 (MOSI) │ 19 │ 20 │ Ground
//!                  ├────┼────┤
//!    GPIO 9 (MISO) │ 21 │ 22 │ GPIO 25
//!                  ├────┼────┤
//!   GPIO 11 (SCLK) │ 23 │ 24 │ GPIO 8 (CE0)
//!                  ├────┼────┤
//!           Ground │ 24 │ 26 │ GPIO 7 (CE1)
//!                  ├────┼────┤
//!   GPIO 0 (ID_SD) │ 27 │ 28 │ GPIO 1 (ID_SC)
//!                  ├────┼────┤
//!           GPIO 5 │ 29 │ 30 │ Ground
//!                  ├────┼────┤
//!           GPIO 6 │ 31 │ 32 │ GPIO 12 (PWM0)
//!                  ├────┼────┤
//!   GPIO 13 (PWM1) │ 33 │ 34 │ Ground
//!                  ├────┼────┤
//! GPIO 19 (PCM_FS) │ 35 │ 36 │ GPIO 16
//!                  ├────┼────┤
//!          GPIO 26 │ 37 │ 38 │ GPIO 20 (PCM_DIN)
//!                  ├────┼────┤
//!           Ground │ 39 │ 40 │ GPIO 21 (PCM_DOUT)
//!                  └────┴────┘
//!                 Ethernet side
