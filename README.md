 ## Library for retrieving readings from Adafruit STEMMA Soil Sensor.

 The implementation is based on the [Adafruit CircuitPython Seesaw library](https://github.com/adafruit/Adafruit_CircuitPython_seesaw).

 The library is tested and used on a Raspberry Pi 3 B+ board, running Raspbian but uses interfaces
 from `embedded_hal` operations like sleep/delay and other system calls.

 ## Example

 ```rust
 pub fn main(interval_ms: u64) {
    use stemma_soil_sensor::SoilSensor;
    use linux_embedded_hal::Delay;
    use embedded_hal::blocking::delay::DelayMs;

    let delay = Delay {};
    let mut sensor = SoilSensor::init(delay).unwrap();

     loop {
        let temp = sensor.get_temp().unwrap();
        let cap = sensor.get_capacitance().unwrap();
        println!("Temperature: {:.02}", temp);
        println!("Capacitance: {}", cap);
        let mut delay = Delay {};
        delay.delay_ms(2000u32);
    }
}
 ```

## Using this library with other boards

Since this library relies on [rppal](https://github.com/golemparts/rppal/tree/master/src) for the
I2C communication it won't work with other boards than the Raspberry Pi series boards as it is
right now.

## Requirements

This library should build on the following targets:

  - armv7-unknown-linux-gnueabihf
  - armv7-unknown-linux-musleabihf
  - aarch64-unknown-linux-gnu

## Additional notes

Please make a note of that the repository includes a `.cargo/config` entry. This is excluded from
the crate package. If you use this library from the repository directly make sure to change this
(or remove it) so it doesn't cause problems for your build.
