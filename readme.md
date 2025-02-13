
# esp-ota-std-example
This is an example to demonstrate use of the esp-ota crate to perform an over-the-air update. The second boot image 
is currently stored in static memory but would theoretically have been uploaded through a web server.

Notes
- I'm running an Arduino Nano ESP32 
- The esp-ota needs to be updates to esp-idf-sys 0.35. I maintain a local copy that has been modified until that happens. A PR has been submitted.

References
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
