# TCA9535 I/O Expander embedded-hal i2c driver

From the [datasheet](https://www.ti.com/lit/ds/symlink/tca9535.pdf): 

The TCA9535is a 24-pin device that provides 16 bits of general purpose
parallel input and output (I/O) expansion for the two-line bidirectional I2C bus or
(SMBus) protocol. The device can operate with a power supply voltage ranging from
1.65 V to 5.5 V.


## Usage

Include the library as a dependency in your Cargo.toml

[dependencies.tca9535]
version = "0.1"

Use `embedded-hal` implementation to get the i2c bus for the chip then
create the handle.  


```rust
use tca9535::{Tca9535, Address, Port};

// Obtain an i2c bus using embedded-hal
// let mut i2c = ...;

// Note that the handle does *not* capture the i2c bus
// object and that the i2c bus object must be supplied for each method call.
// This slight inconvenience allows sharing the i2c bus between peripherals.
let gpio = Tca9535::new(&i2c, Address::ADDR_0x20);

// Set all outputs low
gpio.clear_outputs(&mut i2c)?;

// Set port P01 and P05 as inputs and the rest as outputs.
gpio.write_config(&mut i2c, Port::P01 | Port::P05)?;

// Read pin input logic levels
let inputs = gpio.read_inputs(&mut i2c)?;

// Do something if port P01 is high:
if inputs & Port::P01 {
    // ...
}
 ```
