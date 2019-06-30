#![no_std]
#![no_main]

extern crate panic_halt;

use hifive1::hal::e310x::Peripherals;
use hifive1::hal::prelude::*;
use hifive1::hal::spi::{Spi, MODE_0};
use hifive1::sprintln;
use mfrc522::Mfrc522;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO0.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, gpio.pin17, gpio.pin16, 115_200.bps(), clocks);

    // Configure SPI pins
    let mosi = gpio.pin3.into_iof0();
    let miso = gpio.pin4.into_iof0();
    let sck = gpio.pin5.into_iof0();
    //let cs = gpio.pin2.into_iof0();
    let cs = gpio.pin2.into_output();

    // Configure SPI
    let pins = (mosi, miso, sck /*cs*/);
    let spi = Spi::new(p.QSPI1, pins, MODE_0, 1_000_000.hz(), clocks);

    // Configure MFRC522
    //let pin = DummyPin {};
    let mut mfrc522 = Mfrc522::new(spi, cs).unwrap();

    sprintln!("starting");

    let vers = mfrc522.version().unwrap();

    sprintln!("VERSION: 0x{:x}", vers);

    assert!(vers == 0x91 || vers == 0x92);

    loop {
        if let Ok(atqa) = mfrc522.reqa() {
            if let Ok(uid) = mfrc522.select(&atqa) {
                sprintln!("UID: {:?}", uid);
            }
        }
    }
}
