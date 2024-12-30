//!
//! main.rs
//! surfingreg
//!
//! Ref:
//! https://github.com/faern/esp-ota/blob/aa51f220b0bfaf73d6142faea5e9472e0f106788/examples/ota_from_flash.rs
//!

const APP_VERSION: &str = "alpha";

fn main() {
    println!("[main] Starting {APP_VERSION}...!");
    println!("[main error] Starting {APP_VERSION}...!");
    // esp_idf_svc::sys::link_patches();
    esp_idf_sys::link_patches();
    // esp_idf_svc::log::EspLogger::initialize_default();

    if APP_VERSION == "beta" {
        println!("[beta::main] hello from ota beta");
    } else {
        let NEW_APP: &[u8] = include_bytes!("../beta.bin");
        println!("[alpha::main] writing beta.bin...");
        let mut ota = esp_ota::OtaUpdate::begin().unwrap();

        for app_chunk in NEW_APP.chunks(4096) {
            if let Err(err) = ota.write(app_chunk) {
                println!("[main error] Failed to write chunk");
                break;
            }
        }

        //validate the written app
        match ota.finalize() {
            Err(e) => {
                println!("[main error] Failed to validate image. {:?}", e);
                ()
            }
            Ok(mut x) => {
                println!("[main] setting boot partition...");
                x.set_as_boot_partition().unwrap();
                println!("[main] rebooting...");
                x.restart();
            }
        };
    }

    println!("[main] Done: {APP_VERSION}...!");
}
