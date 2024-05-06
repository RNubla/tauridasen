use core::time;
use std::{error::Error, time::Duration};

use btleplug::{
    api::{Central, Manager as _, Peripheral, ScanFilter},
    platform::Manager,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// async fn scan_for_devices() -> Result<(), btleplug::Error> {
//     let manager = Manager::new().await?;
//     let adapter_list = manager.adapters().await?;
//     if adapter_list.is_empty() {
//         eprintln!("No Bluetooth adapters found")
//     }
//     for adapter in adapter_list.iter() {
//         println!("Starting scan on {}...", adapter.adapter_info().await?);
//         adapter
//             .start_scan(ScanFilter::default())
//             .await
//             .expect("Can't scan BLE adapter for connected devices...");
//         // time::sleep(Duration::from_secs(10)).await;
//         let peripherals = adapter.peripherals().await?;
//         if peripherals.is_empty() {
//             eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
//         } else {
//             // All peripheral devices in range
//             for peripheral in peripherals.iter() {
//                 let properties = peripheral.properties().await?;
//                 let is_connected = peripheral.is_connected().await?;
//                 let local_name = properties
//                     .unwrap()
//                     .local_name
//                     .unwrap_or(String::from("(peripheral name unknown)"));
//                 println!(
//                     "Peripheral {:?} is connected: {:?}",
//                     local_name, is_connected
//                 );
//                 if !is_connected {
//                     println!("Connecting to peripheral {:?}...", &local_name);
//                     if let Err(err) = peripheral.connect().await {
//                         eprintln!("Error connecting to peripheral, skipping: {}", err);
//                         continue;
//                     }
//                 }
//                 let is_connected = peripheral.is_connected().await?;
//                 println!(
//                     "Now connected ({:?}) to peripheral {:?}...",
//                     is_connected, &local_name
//                 );
//                 peripheral.discover_services().await?;
//                 println!("Discover peripheral {:?} services...", &local_name);
//                 for service in peripheral.services() {
//                     println!(
//                         "Service UUID {}, primary: {}",
//                         service.uuid, service.primary
//                     );
//                     for characteristic in service.characteristics {
//                         println!("  {:?}", characteristic);
//                     }
//                 }
//                 if is_connected {
//                     println!("Disconnecting from peripheral {:?}...", &local_name);
//                     peripheral
//                         .disconnect()
//                         .await
//                         .expect("Error disconnecting from BLE peripheral");
//                 }
//             }
//         }
//     }
//     Ok(())
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
