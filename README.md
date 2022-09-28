# USB playground

## Run

### Phase 3: dioxus

That is WIP!

Works with a host program, turns on the light on and off and gives temperature with an GUI made (poorly) with Dioxus.
1. Host is in its folder: `cd host` and `cargo run --bin app`
2. The board: `cd firmware` and then `DEFMT_LOG=info cargo rb usb`
3. Click `on`, `off` or `temp` on the window to see the bord turning on the led, or turning off the led, or giving the temperature.

Error handling is still left as an exercise...
