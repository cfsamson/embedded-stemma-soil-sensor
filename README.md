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
        println!("The temperature is: {:.02}", temp);
        println!("The capactiance is: {}", cap);
        let mut delay = Delay {};
        delay.delay_ms(2000u32);
    }
}
 ```

## Using this library with other boards

There are minor differences between this implementation and the one for the Arduino board. The two
major differences are:

1. Reading output from the sensor is **not** done in 32 byte chunks. This _could_ pose an issue.
2. The timings to allow the sensor to process are slightly longer than the C implementation and
is based on the Python library. This should not matter.

If #1 is a problem I'll be positive to implement a `read` method which allows the user of the
library to supply a buffer of the size they need. Leave an issue or a PR with the implementation
and I'll add it.

On #2, the short timings used in the C example continuously caused read errors when I tried it so
I'm not sure lowering it is possible.
