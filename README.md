# USB playground

## Run

### Phase 1: coolterm

Works with coolterm and echoes back `a .. z` to capital letters
1. start coolterm, identify your usb.
    Tips: run `ls -l /dev/serial/by-id` will return the port of the `usb-SEGGER_J-Link`. You need the other one (Usually `ACMx` on Linux, `x=0,1,2..`).
2. both USB and Seger probe must be connected with nRF52840!

3. Run `DEFMT_LOG=info cargo rb usb` in the firmware folder.
4. On coolterm, find right port (should be `/dev/ttyACM{0/1/2}` as per point 1. on Linux)
5. Type letters in coolterm, they should be echoed back in capital letters.

The example is taken from USB serial example in [`nrf-hal`](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/usb_serial.rs).

![](example.png)

\* that info is in the `.cargo/config.toml` file but if it does not publish any letter, use explicit logging.

[.cargo/config.toml]
```toml
[env]
DEFMT_LOG="info"
```

### Phase 2:

Works with a host program
1. Do `cd host` and `cargo run --bin send`
2. In the `cd firmware` and then `DEFMT_LOG=info cargo rb usb`

The host is sending a serialized command to the firmware.



### Phase 3: on/off/temp

Works with a host program, turns on the light on and off and gives temperature
1. Do `cd host` and `cargo run --bin send`
2. In the `cd firmware` and then `DEFMT_LOG=info cargo rb usb`
3. Write `on`, `off` or `temp` to the `host` to see the bord turning on the led, or turning off the led, or giving the temperature.

Error handling is left as an exercise ... to myself for later.

