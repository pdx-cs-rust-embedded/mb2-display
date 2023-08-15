#![no_main]
#![no_std]

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::pixelcolor::Rgb565;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use st7735_lcd::ST7735;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{delay::Delay, gpio, spim},
    pac::spim0::frequency,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut delay = Delay::new(board.SYST);

    let spi_mode = spim::Mode {
        polarity: spim::Polarity::IdleLow,
        phase: spim::Phase::CaptureOnFirstTransition,
    };
    // XXX Can probably be much faster.
    let spi_clock_rate = frequency::FREQUENCY_A::M8;
    let sck = board.pins.p0_17
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let mosi = board.pins.p0_13
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let spi_pins = spim::Pins {
        sck,
        mosi: Some(mosi),
        miso: None,
    };
    let spi = spim::Spim::new(board.SPIM0, spi_pins, spi_clock_rate, spi_mode, 0xff);

    let dc = board.pins.p0_10
        .into_push_pull_output(gpio::Level::High)
        .degrade();
    let rst = board.pins.p0_09
        .into_push_pull_output(gpio::Level::High)
        .degrade();
    let mut display = ST7735::new(
        spi,
        dc,
        rst,
        true, // rgb
        true, // inverted
        160, // width
        80, // height
    );

    display.init(&mut delay).unwrap();
    display.set_orientation(&st7735_lcd::Orientation::Landscape).unwrap();
    display.set_offset(0, 25);
    display.clear(Rgb565::WHITE).unwrap();

    // Circle with styled stroke and fill with top-left point at (50, 20) with a diameter of 30
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(3)
        .fill_color(Rgb565::GREEN)
        .build();

    Circle::new(Point::new(80, 30), 60)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    loop{
        continue;
    }
}
