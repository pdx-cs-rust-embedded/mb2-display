#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{gpio, spi, spim},
    pac::{self, spi0::frequency},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = pac::Peripherals::take().unwrap();
    let board = Board::take().unwrap();
    let spi_mode = spim::Mode {
        polarity: spim::Polarity::IdleLow,
        phase: spim::Phase::CaptureOnFirstTransition,
    };
    // XXX Can probably be much faster.
    let spi_clock_rate = frequency::FREQUENCY_A::K125;
    let sck = board.pins.p0_17
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let mosi = board.pins.p0_13
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let spi_pins = spi::Pins {
        sck,
        mosi: Some(mosi),
        miso: None,
    };
    let _spi = spi::Spi::new(p.SPI0, spi_pins, spi_clock_rate, spi_mode);

    loop{
        continue;
    }
}
