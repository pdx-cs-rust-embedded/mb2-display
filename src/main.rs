#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{spi::Spi, spim},
    pac::spim0,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let spi_mode = spim::Mode {
        polarity: spim::Polarity::IdleLow,
        phase: spim::Phase::CaptureOnFirstTransition,
    };
    // XXX Can probably be much faster.
    let spi_clock_rate = spim0::FREQUENCY_A::K125;
    let spi = Spi::new(board.SPIM0, );
}
