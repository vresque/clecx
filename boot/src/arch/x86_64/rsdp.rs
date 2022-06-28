use bkshared::rsdp::RsdpWrapper;
use efi::{
    println,
    raw::{
        config::ConfigurationTable,
        guid::{ACPI_20_TABLE_GUID, ACPI_TABLE_GUID},
    },
    status::{EfiResult, Status},
    table,
    vec::Vec,
};

pub fn find_rsdps() -> EfiResult<Vec<RsdpWrapper>> {
    let table = table();
    let mut v: efi::vec::Vec<RsdpWrapper> = efi::vec![];
    let mut configs = unsafe {
        core::slice::from_raw_parts::<ConfigurationTable>(
            table.configuration,
            table.number_of_table_entries,
        )
    };
    for cfg in configs {
        if matches!((*cfg).vendor_guid, ACPI_TABLE_GUID | ACPI_20_TABLE_GUID) {
            match RsdpWrapper::validate(cfg.vendor_table as u64) {
                Ok(rs) => v.push(rs),
                Err(e) => println!(
                    "Found invalid RSDP Pointer at {}, Reason for dismissal: {:?}",
                    cfg.vendor_table, e
                ),
            }
        }
    }
    if v.len() == 0 {
        return Err(Status::Unsupported);
    }
    Ok(v)
}
