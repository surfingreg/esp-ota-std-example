
# esp-ota-std-example

- Naive implementation of OTA without actually addressing how a second image gets there. Currently loaded in memory. 
- Exercise the esp-ota crate (assuming it's been updated to esp-idf-sys 0.35)
- todo: This sample always boots from bravo after the first run; basically no way to update without messing with partitions.
It's not running the newly flashed (alpha) partition but is rather just re-running the bravo partition that was flashed on the first run of alpha.  
- 

References
- https://github.com/faern/esp-ota/tree/aa51f220b0bfaf73d6142faea5e9472e0f106788
- https://github.com/DaneSlattery/esp-ota/tree/main/examples
- https://github.com/esp-rs/espflash/issues/156
- https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/system/ota.html
- https://github.com/johnthagen/min-sized-rust
- 
- esp-idf-svc OTF
- https://github.com/esp-rs/esp-idf-svc/blob/34dfee2a4f08692f415e965e4cd117034e6be56e/src/ota.rs

espflash config for 16MB
- https://docs.rs/crate/cargo-espflash/latest
