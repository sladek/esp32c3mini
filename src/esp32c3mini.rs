use embedded_hal::watchdog::WatchdogDisable;
//use esp32c3_hal::system::SystemParts;
use esp32c3_hal::{
    clock::ClockControl, gpio::IO, pac::Peripherals, prelude::*, spi::Spi, system::SystemExt,
    timer::TimerGroup, Delay, Rtc,
};

use smart_leds::{SmartLedsWrite, RGB};
use ws2812_spi::Ws2812;

pub struct Esp32c3mini {
    ws: Ws2812<Spi<esp32c3_hal::pac::SPI2>>,
    delay: Delay,
}

impl Esp32c3mini {
    pub fn new() -> Self {
        //        let p = Peripherals::take();
        //        let spi: Spi<esp32c3_hal::pac::SPI2>;
        let peripherals = Peripherals::take().unwrap();
        let mut system = peripherals.SYSTEM.split();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
        let mosi = io.pins.gpio8;
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

        // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT and the RTC WDT.
        let mut rtc = Rtc::new(peripherals.RTC_CNTL);
        let mut timer0 = TimerGroup::new(peripherals.TIMG0, &clocks);
        let mut timer1 = TimerGroup::new(peripherals.TIMG1, &clocks);

        timer0.wdt.disable();
        timer1.wdt.disable();
        rtc.swd.disable();
        rtc.rwdt.disable();

        let spi = esp32c3_hal::spi::Spi::new_mosi_only(
            peripherals.SPI2,
            mosi,
            fugit::HertzU32::Hz(3_333_333),
            esp32c3_hal::spi::SpiMode::Mode0,
            &mut system.peripheral_clock_control,
            &clocks,
        );
        Self {
            ws: Ws2812::new(spi),
            delay: Delay::new(&clocks),
        }
    }

    pub fn rgb_write(&mut self, color: RGB<u8>) {
        let data: [RGB<u8>; 1] = [color];
        self.ws.write(data.iter().cloned()).unwrap();
    }
    pub fn delay_ms(&mut self, delay_ms: u32) {
        self.delay.delay_ms(delay_ms);
    }
}
