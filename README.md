# USB playground

## Run

### Phase 1:

Works with coolterm.
1. start coolterm
2. both USB and probe must be connected with nRF52840

3. `DEFMT_LOG=info cargo rb usb`*
4. on coolterm, find right port (should be `/dev/ttyACM{0/1/2}` on Linux)
5. type letters in coolterm

PS: Words of the song are from Kelly Clarkson "Born to die".

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
2. In the `cd firmware` and then `DEFMT_LOG=info cargo rb usb`*

The host is sending a serialized command to the firmware.



\* that info is in the `.cargo/config.toml` file but if it does not publish any letter, use explicit logging.

[.cargo/config.toml]
```toml
[env]
DEFMT_LOG="info"
```