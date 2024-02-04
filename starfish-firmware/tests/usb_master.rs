//! This detects whether the RP Pico is plugged in via
//! USB (Master controller) or via VSYS power (Slave controller)
//! Turns the LED on if master, and keeps it off otherwise.
#![no_std]
#![no_main]

use embedded_hal::digital::v2::PinState;
use rp_pico as bsp;

use bsp::{
    entry,
    hal::{clocks::init_clocks_and_plls, Sio, Watchdog},
    pac,
};
use defmt_rtt as _;
use panic_probe as _;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let _clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
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

    // Reset bit clear: needed to be able to read the status register
    pac.RESETS.reset.modify(|_, w| w.dma().clear_bit());

    // Find the register in memory and read as boolean
    let ptr = (0x50110000 + 0x50) as *mut u32;
    let reg_sie_status: u32 = unsafe { ptr.read_volatile() };
    let is_master = reg_sie_status & (1u32 << 16u32) == 0;

    // Pre-runtime checking
    let mut _pin_led = match is_master {
        true => pins.led.into_push_pull_output_in_state(PinState::High),
        false => pins.led.into_push_pull_output_in_state(PinState::Low),
    };

    // Runtime checking
    // let mut pin_led = pins.led.into_push_pull_output();
    // if is_master {
    //     pin_led.set_low().unwrap();
    // } else {
    //     pin_led.set_high().unwrap();
    // }

    loop {}
}
