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
        false, // BGR, not RGB
        true, // inverted (color, not direction)
        160, // width
        80, // height
    );

    #[cfg(feature = "dim")]
    {
        use microbit::hal::pwm;

        let brightness_pin = board.pins.p1_02.into_push_pull_output(gpio::Level::High);
        let brightness = pwm::Pwm::new(board.PWM0);
        brightness
            .set_output_pin(pwm::Channel::C0, brightness_pin.degrade())
            .set_prescaler(pwm::Prescaler::Div128)
            .set_counter_mode(pwm::CounterMode::Up)
            .set_max_duty(1000)
            .enable();
        brightness.set_duty_off_common(100);
    }

    display.init(&mut delay).unwrap();
    display.set_orientation(&st7735_lcd::Orientation::Landscape).unwrap();
    display.set_offset(0, 25);
    display.clear(Rgb565::WHITE).unwrap();

    // Circle with styled stroke and fill with top-left point at (50, 20) with a diameter of 30
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::BLUE)
        .stroke_width(3)
        .fill_color(Rgb565::GREEN)
        .build();

    Circle::new(Point::new(50, 10), 60)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    loop{
        continue;
    }
}
