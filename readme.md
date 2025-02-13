
# esp-ota-std-example
This is an example to demonstrate use of the esp-ota crate to perform an over-the-air update. The second boot image 
is currently stored in static memory but would theoretically have been uploaded through a web server.

## Notes
- I'm running an Arduino Nano ESP32 
- The esp-ota needs to be updates to esp-idf-sys 0.35. I maintain a local copy that has been modified until that happens. A PR has been submitted.
- bin.sh generates the second binary to be uploaded. 
- Running this project via flash.sh performs the intial flash of the ESP32 connected via USB. The idea is it can then be disconnected and accessed over wifi.

## References
- https://github.com/faern/esp-ota/tree/aa51f220b0bfaf73d6142faea5e9472e0f106788
- https://github.com/DaneSlattery/esp-ota/tree/main/examples
- https://github.com/esp-rs/espflash/issues/156
- https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/system/ota.html
- https://github.com/johnthagen/min-sized-rust
- 
- esp-idf-svc OTF
- https://github.com/esp-rs/esp-idf-svc/blob/34dfee2a4f08692f415e965e4cd117034e6be56e/src/ota.rs

- espflash config for 16MB
- https://docs.rs/crate/cargo-espflash/latest

## TODO
[] remove the esp-ota crate (same functionality now seems better documented in esp-itf-svc and esp-ota is just a wrapper)  
https://docs.rs/crate/esp-ota/0.2.2)
[ ] add an axum web server
[ ] connect to wifi; access the web server disconnected from USB
[ ] create an example to build the two different binaries a/b