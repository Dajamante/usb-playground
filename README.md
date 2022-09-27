# USB playground

## Run

### Phase 1:

Works with coolterm and echoes back `a .. z` to capital letters
1. Go to the branch `coolterm`
2. start coolterm, identify your usb.
    Tips: run `ls -l /dev/serial/by-id` will return the port of the `usb-SEGGER_J-Link`. You need the other one (Usually `ACMx` on Linux, `x=0,1,2..`).
3. both USB and Seger probe must be connected with nRF52840!

4. Run `DEFMT_LOG=info cargo rb usb` in the firmware folder.
5. On coolterm, find right port (should be `/dev/ttyACM{0/1/2}` as per point 1. on Linux)
6. Type letters in coolterm, they should be echoed back in capital letters.

![](example.png)

\* that info is in the `.cargo/config.toml` file but if it does not publish any letter, use explicit logging.

[.cargo/config.toml]
```toml
[env]
DEFMT_LOG="info"
```
