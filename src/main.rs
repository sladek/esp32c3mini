#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

//use esp_println::println;
use smart_leds::colors::*;

mod esp32c3mini;

#[entry]
fn main() -> ! {
    const DELAY_MILISECONDS: u32 = 500;
    let mut esp32 = esp32c3mini::Esp32c3mini::new();

    loop {
        esp32.rgb_write(RED);
        esp32.delay_ms(DELAY_MILISECONDS);
        esp32.rgb_write(BLUE);
        esp32.delay_ms(DELAY_MILISECONDS);
        esp32.rgb_write(GREEN);
        esp32.delay_ms(DELAY_MILISECONDS);
        esp32.rgb_write(ORANGE);
        esp32.delay_ms(DELAY_MILISECONDS);
        esp32.rgb_write(BROWN);
        esp32.delay_ms(DELAY_MILISECONDS);
    }
}
