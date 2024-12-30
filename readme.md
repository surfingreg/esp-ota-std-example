
# esp-ota-std-example

- Naive implementation of OTA without actually addressing how a second image gets there. Currently loaded in memory. 
- Exercise the esp-ota crate (assuming it's been updated to esp-idf-sys 0.35)

## TODO
- currently doesn't withstand a second attempt to load "alpha"; maybe call set_as_boot_partition() initially 


References
- https://github.com/faern/esp-ota/tree/aa51f220b0bfaf73d6142faea5e9472e0f106788
- https://github.com/DaneSlattery/esp-ota/tree/main/examples
- https://github.com/esp-rs/espflash/issues/156
- https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/system/ota.html
- https://github.com/johnthagen/min-sized-rust