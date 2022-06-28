#![no_std]
#![no_main]
#![feature(never_type)]

use efi::raw::key::key;
use efi::raw::runtime::ResetType;
use efi::raw::table::Table;
use efi::status::Status;
use efi::{println, setup, table, Handle};

mod arch;

#[no_mangle]
pub extern "win64" fn efi_main(handle: Handle, sys_table: *mut Table) -> Status {
    setup(sys_table, handle);
    println!("Booting Clecx...");
    // Set the watchdog timer (So that it doesn't shut down after 5min)
    (table().boot.set_watchdog_timer)(0, 0, 0, core::ptr::null());

    match arch::main() {
        Ok(_) => unreachable!(), // Result<!, Status> --> Does not return on success
        Err(e) => {
            println!("Error: {:#?}", e);
            println!("Press any key to continue");
            let _ = key(true);
            // Wait for keypress
        }
    }

    (table().runtime.reset_system)(ResetType::Cold, Status::Aborted, 0, core::ptr::null())
}
