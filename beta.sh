
. /home/gp/export-esp.sh


# rm beta.bin
cargo build --release
export CHIP=esp32s3
export TARGET=xtensa-esp32s3-espidf
export MYAPP=esp-ota-std-ex
espflash save-image --chip ${CHIP} target/${TARGET}/release/${MYAPP} beta.bin


# espflash flash --monitor --partition-table=part_4mb.csv target/xtensa-esp32s3-espidf/release/esp-ota-std-ex
