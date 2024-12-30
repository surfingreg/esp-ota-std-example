//!
//! main.rs
//! surfingreg
//!

// const APP_VERSION: &'static str = "bravo";
const APP_VERSION: &'static str = "alpha";


fn main()-> Result<(), Box<dyn std::error::Error>> {
    esp_idf_sys::link_patches();
    if APP_VERSION == "bravo" {
        println!("[main {}] hello", APP_VERSION);
    //     esp_ota::rollback_and_reboot().expect("Failed to roll back to working app");
    } else {
        println!("[main {}] hello", APP_VERSION);
        let new_app: &[u8] = include_bytes!("../beta.bin");
        println!("[main {APP_VERSION}] writing beta.bin...");
        let mut ota = esp_ota::OtaUpdate::begin()?; // .unwrap();
        for app_chunk in new_app.chunks(4096){
            ota.write(app_chunk)?
        }
        // Unless you also call set_as_boot_partition the new app will not start.
        let mut completed_ota = ota.finalize()?;

        // Sets the newly written to partition as the next partition to boot from.
        println!("[main {APP_VERSION}] setting boot partition...");
        completed_ota.set_as_boot_partition()?;

        // Restarts the CPU, booting into the newly written app.
        println!("[main {APP_VERSION}] restarting...");
        completed_ota.restart();

    }

    Ok(())
}
