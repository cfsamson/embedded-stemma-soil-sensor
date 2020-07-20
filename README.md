 ## Library for retrieving readings from Adafruit STEMMA Soil Sensor.

 The implementation is based on the [Adafruit CircuitPython Seesaw library](https://github.com/adafruit/Adafruit_CircuitPython_seesaw).

 The library is tested and used on a Raspberry Pi 3 B+ board, running Raspbian but uses interfaces
 from `embedded_hal` for all operation so it should work in `no_std` environments as well.

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
