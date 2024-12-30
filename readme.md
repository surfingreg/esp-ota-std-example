

espflash flash --partition-table partitions.csv --monitor target/xtensa-esp32s3-espidf/release/esp-ota-std-ex



error: failed to select a version for `esp-idf-sys`.
... required by package `esp-ota v0.2.0`
... which satisfies dependency `esp-ota = "^0.2.0"` of package `esp-ota-std-ex v0.1.0 (/home/gp/Dropbox/dev/rust/esp-ota/esp-ota-std-ex)`
versions that meet the requirements `^0.33.0` are: 0.33.7, 0.33.6, 0.33.5, 0.33.4, 0.33.3, 0.33.2, 0.33.1, 0.33.0

the package `esp-idf-sys` links to the native library `esp_idf`, but it conflicts with a previous package which links to `esp_idf` as well:
package `esp-idf-sys v0.35.0`
... which satisfies dependency `esp-idf-sys = "^0.35"` of package `esp-idf-hal v0.44.1`
... which satisfies dependency `esp-idf-hal = "^0.44"` of package `esp-idf-svc v0.49.0`
... which satisfies dependency `esp-idf-svc = "^0.49"` of package `esp-ota-std-ex v0.1.0 (/home/gp/Dropbox/dev/rust/esp-ota/esp-ota-std-ex)`
Only one package in the dependency graph may specify the same links value. This helps ensure that only one copy of a native library is linked in the final binary. Try to adjust your dependencies so that only one package uses the `links = "esp_idf"` value. For more information, see https://doc.rust-lang.org/cargo/reference/resolver.html#links.

failed to select a version for `esp-idf-sys` which could resolve this conflict
