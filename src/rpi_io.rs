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
//!  GPIO 4 (GPCLK0) │ 07 │  8 │ GPIO 14 (TXD) LED - RED
//!                  ├────┼────┤
//!           Ground │ 09 │ 10 │ GPIO 15 (RXD) LED - BLUE
//!                  ├────┼────┤
//!          GPIO 17 │ 11 │ 12 │ GPIO 18 (PCM_CLK) Left button
//!                  ├────┼────┤
//!          GPIO 27 │ 13 │ 14 │ Ground
//!                  ├────┼────┤
//!          GPIO 22 │ 15 │ 16 │ GPIO 23 - Central button
//!                  ├────┼────┤
//!        3v3 power │ 17 │ 18 │ GPIO 24 - Right button
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

use std::{error::Error, time::Duration};

use rppal::gpio::{Event, Gpio, InputPin, OutputPin};
use tokio::sync::Mutex;

const LED_CONNECTED_GPIO_NB: u8 = 14;
const LED_DISCONNECTED_GPIO_NB: u8 = 15;
const BUTTON_BACKWARD_GPIO_NB: u8 = 18;
const BUTTON_PAUSE_GPIO_NB: u8 = 23;
const BUTTON_FORWARD_GPIO_NB: u8 = 24;

pub struct RpiIo {
    led_connected: Mutex<OutputPin>,
    led_disconnected: Mutex<OutputPin>,
    button_forward: Mutex<InputPin>,
    button_pause: Mutex<InputPin>,
    button_backward: Mutex<InputPin>,
}

impl RpiIo {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            led_connected: Mutex::new(
                Gpio::new()?.get(LED_CONNECTED_GPIO_NB)?.into_output(),
            ),
            led_disconnected: Mutex::new(
                Gpio::new()?.get(LED_DISCONNECTED_GPIO_NB)?.into_output(),
            ),
            button_forward: Mutex::new(
                Gpio::new()?
                    .get(BUTTON_FORWARD_GPIO_NB)?
                    .into_input_pulldown(),
            ),
            button_pause: Mutex::new(
                Gpio::new()?
                    .get(BUTTON_PAUSE_GPIO_NB)?
                    .into_input_pulldown(),
            ),
            button_backward: Mutex::new(
                Gpio::new()?
                    .get(BUTTON_BACKWARD_GPIO_NB)?
                    .into_input_pulldown(),
            ),
        })
    }

    pub async fn led_connected_set_high(&mut self) {
        self.led_connected.lock().await.set_high();
    }

    pub async fn led_connected_set_low(&mut self) {
        self.led_connected.lock().await.set_low();
    }

    pub async fn led_disconnected_set_high(&mut self) {
        self.led_disconnected.lock().await.set_high();
    }

    pub async fn led_disconnected_set_low(&mut self) {
        self.led_disconnected.lock().await.set_low();
    }

    pub async fn button_forward_set_async_interrupt<C>(
        &mut self,
        callback: C,
    ) -> Result<(), Box<dyn Error>>
    where
        C: FnMut(Event) + Send + 'static,
    {
        Ok(self.button_forward.lock().await.set_async_interrupt(
            rppal::gpio::Trigger::RisingEdge,
            Some(Duration::from_millis(50)),
            callback,
        )?)
    }

    pub async fn button_pause_set_async_interrupt<C>(
        &mut self,
        callback: C,
    ) -> Result<(), Box<dyn Error>>
    where
        C: FnMut(Event) + Send + 'static,
    {
        Ok(self.button_pause.lock().await.set_async_interrupt(
            rppal::gpio::Trigger::RisingEdge,
            Some(Duration::from_millis(50)),
            callback,
        )?)
    }

    pub async fn button_backward_set_async_interrupt<C>(
        &mut self,
        callback: C,
    ) -> Result<(), Box<dyn Error>>
    where
        C: FnMut(Event) + Send + 'static,
    {
        Ok(self.button_backward.lock().await.set_async_interrupt(
            rppal::gpio::Trigger::RisingEdge,
            Some(Duration::from_millis(50)),
            callback,
        )?)
    }
}
