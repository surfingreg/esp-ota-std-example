//!
//! main.rs
//! surfingreg
//!

use esp_idf_svc::ota::EspOta;
use esp_idf_svc::partition::{EspPartitionIterator};

#[cfg(feature = "bravo")]
const APP_VERSION: &'static str = "bravo";

#[cfg(feature = "alpha")]
const APP_VERSION: &'static str = "alpha";

#[cfg(feature = "alpha")]
const NEW_APP: &[u8] = include_bytes!("../beta.bin");

/// todo: transition to sys api 
/// todo: need a repeatable way to push to the same partition (multiple ota partitions just alternate and don't 
/// boot the the one being flashed.
fn check_partitions(){
    let part_iterator = unsafe { EspPartitionIterator::new(None).unwrap() };
    println!("[check_partitions] {} partitions:", APP_VERSION);
    for part in part_iterator {
        println!("[check_partitions] label: {}, type: {:?}, addr: {}, size: {}, erase_size: {}", part.label(), part.partition_type(), part.address(), part.size(), part.erase_size());
        // let offset = 0;    
        // let size = 0;
        // part.erase(offset, size).unwrap()
        // let mut esp = EspOta::new().expect("[main]");
        // println!("factory reset result: {:?}", esp_idf_svc::ota::EspOta::factory_reset(&mut esp));
        

    }
}

fn main()-> Result<(), Box<dyn std::error::Error>> {

    esp_idf_svc::sys::link_patches();
    check_partitions();


    
    if APP_VERSION == "bravo" { 
        println!("[main {}] hello, bravo loaded, marking bravo invalid and rolling back...", APP_VERSION);
        let mut esp = EspOta::new().expect("[main]");
        println!("[main {}] mark_running_slot_invalid_and_reboot: {:?}", APP_VERSION, esp.mark_running_slot_invalid_and_reboot());
        
    
    } else {
    
        #[cfg(feature = "alpha")]
        {
            println!("[main {}] hello", APP_VERSION);
            // esp_ota::mark_app_valid();
    
            println!("[main {APP_VERSION}] writing beta.bin...");
            let mut ota = esp_ota::OtaUpdate::begin()?; // .unwrap();
            for app_chunk in NEW_APP.chunks(4096) {
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
    }
    Ok(())
}
