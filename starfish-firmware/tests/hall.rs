//! This tests the hall effect sensor by turning on
//! the LED when a magnet is detected
//! The Hall Effect Sensor should be plugged into GP1, GND, and GP2
//!
//! When using a single magnet, the sensor gets stuck on a high input
//! Need to introduce an opposite magnetic field to "unstick" it and
//! turn off the LED. This can be taken care of in the actual key switch mechanism
#![no_std]
#![no_main]

use rp_pico as bsp;

use bsp::{entry, hal::Clock};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};

use embedded_hal::digital::v2::PinState;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pin_led = pins.led.into_push_pull_output();
    let mut pin_vcc = pins.gpio1.into_push_pull_output_in_state(PinState::High);
    let pin_hall_in = pins.gpio2.into_pull_up_input();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    loop {
        // Power cycling, imitating the power intervals of a key matrix
        pin_vcc.set_high().unwrap();
        delay.delay_ms(100);

        if pin_hall_in.is_high().unwrap() {
            pin_led.set_high().unwrap();
        } else {
            pin_led.set_low().unwrap();
        }

        pin_vcc.set_low().unwrap();
        delay.delay_ms(300);
    }
}
