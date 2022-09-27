# USB playground

## Run

### Phase 2: on_off_temp

This part works with a host (the computer). Type instructions at the terminal.
1. Checkout the branch `on_off_temp`
2. Go in the host folder `cd host` 
3. Run `cargo run --bin send`
4. Go to `cd firmware` and then run `DEFMT_LOG=info cargo rb usb`

The host is sending a serialized command to the firmware, with [Postcard](https://docs.rs/postcard/latest/postcard/).

❗The [serial busy error](https://github.com/serialport/serialport-rs/blob/6542d11235532ec78332e1e6b4986e73b8d55b11/src/lib.rs#L76)(`Error: Error { kind: Unknown, description: "Device or resource busy" }`) is an indication that the resource might still be used by coolterm.


