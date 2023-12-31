# mb2-display: MicroBit 2 Rust demo of Adafruit TFT display
Bart Massey 2023

This crate is a demo of using the Adafruit
[3533](https://www.adafruit.com/product/3533) TFT display
from Rust on the MicroBit v2.

The demo draws a green circle with a blue border against a
white background at the center of the display.

## Wiring

This display was tested on a breadboard plugged into the
Adafruit
[DragonTail](https://www.adafruit.com/product/3695). It was
wired as follows:

| Display Pin | Connection | MB2 Pin |
|------------:|:----------:|:--------|
|         Vin | 3.3V (+)   |         |
|          3V | NC         |         |
|         GND | Gnd (-)    |         |
|         SCK | P13        | p0.17   |
|        MISO | NC         |         |
|        MOSI | P15        | p0.13   |
|       TFTCS | Gnd (-)    |         |
|         RST | P9         | p0.09   |
|          DC | P8         | p0.10   |
|        SDCS | NC         |         |
|         LIT | (P16)      | (p1.02) |

Here, "NC" is "no connection", "P" numbers refer to Dragon
Tail pins, "p" numbers refer to internal GPIO numbering.

The LIT line may be left unconnected, in which case the
display will run at full brightness. If LIT is connected to
P16 and the `dim` feature is enabled when building (enabled
by default), the display backlight will be PWMed at 10% duty
cycle, dimming the display. Note that the brightness is
highly non-linear: 10% duty cycle is still quite
readable. However, the less-bright display is easier to take
pictures or video of.

Because TFTCS is hard-wired on (active low), and SDCS is not
connected and floats off, only the display can be accessed
in this configuration — the SD card cannot. The MISO line is
also left unconnected; this is used only for SD card
reads.

## Build and Run

You'll need Rust and some embedded tools installed to try
this. Install [Rustup](https://rustup.rs/), then start with
these commands (as needed).

    rustup target add thumbv7em-none-eabihf
    rustup component add llvm-tools
    cargo install cargo-binutils
    cargo install probe-run
    cargo install cargo-embed

Finally, power up your wired-up MB2 and say

    cargo embed --release


# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
