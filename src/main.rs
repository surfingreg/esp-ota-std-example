//!
//! main.rs
//! surfingreg
//!

use esp_idf_svc::ota::EspOta;
use esp_idf_svc::partition::{EspPartitionIterator, EspPartitionType};

#[cfg(feature = "bravo")]
const APP_VERSION: &'static str = "bravo";

#[cfg(feature = "alpha")]
const APP_VERSION: &'static str = "alpha";

#[cfg(feature = "alpha")]
const NEW_APP: &[u8] = include_bytes!("../beta.bin");


fn main()-> Result<(), Box<dyn std::error::Error>> {

    esp_idf_svc::sys::link_patches();

    // let part_type = EspPartitionType::Unknown;
    
    let part_iterator = unsafe {
        EspPartitionIterator::new(None)
            .unwrap()
    };
    println!("[main] {} partitions:", APP_VERSION);
    for mut part in part_iterator {
        println!("[main] label: {}, type: {:?}, addr: {}, size: {}, erase_size: {}", part.label(), part.partition_type(), part.address(), part.size(), part.erase_size());
    
        // let offset = 0;    
        // let size = 0;
        // part.erase(offset, size).unwrap()
    
    }

    

    let mut esp = EspOta::new().expect("[main]");
    println!("factory reset result: {:?}", esp_idf_svc::ota::EspOta::factory_reset(&mut esp));


    // 
    // 
    // if APP_VERSION == "bravo" { println!("[main {}] hello, bravo loaded", APP_VERSION);
    //     // esp_ota::rollback_and_reboot().expect("rollback_and_reboot didn't work");
    //     // unsafe { esp_idf_sys::esp_partition_unload_all() }
    // 
    //     // println!("[main {}] marking {} invalide and rebooting...", APP_VERSION, APP_VERSION);
    //     // unsafe {esp_idf_sys::esp_ota_mark_app_invalid_rollback_and_reboot();}
    // 
    // } else {
    // 
    //     #[cfg(feature = "alpha")]
    //     {
    //         println!("[main {}] hello", APP_VERSION);
    //         esp_ota::mark_app_valid();
    // 
    //         println!("[main {APP_VERSION}] writing beta.bin...");
    //         let mut ota = esp_ota::OtaUpdate::begin()?; // .unwrap();
    //         for app_chunk in NEW_APP.chunks(4096) {
    //             ota.write(app_chunk)?
    //         }
    //         // Unless you also call set_as_boot_partition the new app will not start.
    //         let mut completed_ota = ota.finalize()?;
    // 
    //         // Sets the newly written to partition as the next partition to boot from.
    //         println!("[main {APP_VERSION}] setting boot partition...");
    //         completed_ota.set_as_boot_partition()?;
    // 
    //         // Restarts the CPU, booting into the newly written app.
    //         println!("[main {APP_VERSION}] restarting...");
    //         completed_ota.restart();
    // 
    //     }
    // 
    // }

    Ok(())
}
