# USB playground

## Run

### Phase 1:

Works with coolterm.

1. start coolterm
2. both USB and probe must be connected with nRF52840
3. `DEFMT_LOG=info cargo rb usb`
4. on coolterm, find right port (should be `/dev/ttyACM{0/1/2}` on Linux)
5. type letters in coolterm

PS: Words are from Kelly Clarkson "Born to die".

![](example.png)