use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use vhd_shell::vhd::{DynamicDiskHeader, VhdFooter};

fn main() -> io::Result<()> {
    // Open a VHD file in binary mode
    let mut file = File::open("test.vhd")?;

    // Read the VHD Footer into a buffer
    // There is a copy at the beginning of the file and the FOOTER is 512 bytes
    let mut footer = [0u8; 512];
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut footer)?;

    // Before parsing the footer ensure that at least signature is correct.
    if &footer[0..8] == "conectix".as_bytes() {
        let vhd_footer = VhdFooter::parse(&footer).unwrap();
        println!("== VHD FOOTER ==");
        println!(
            "Confirmed that sig is {:?}",
            String::from_utf8_lossy(vhd_footer.signature())
        );
        println!("next offset: {}", vhd_footer.next_offset());
        println!("Disk size  : {}", vhd_footer.disk_size());
        println!("Data size  : {}", vhd_footer.data_size());
        println!(
            "Disk geometry: Cylinders {}/ Heads {}/ Sectors {}",
            vhd_footer.disk_cylinders(),
            vhd_footer.disk_heads(),
            vhd_footer.disk_sectors()
        )
    } else {
        // If we don't find the signature no need to read the dynamic disk header
        println!("Found {:?} instead of conectix", &footer[0..8]);
        return Ok(());
    }

    // Read the dynamic disk header
    let mut dyn_disk_header = [0u8; 1024];
    file.seek(SeekFrom::Start(512))?;
    file.read_exact(&mut dyn_disk_header)?;

    // Before parsing the dynamic disk header ensure that signature is correct.
    if &dyn_disk_header[0..8] == "cxsparse".as_bytes() {
        let vhd_dyn_disk_header = DynamicDiskHeader::parse(&dyn_disk_header).unwrap();
        println!("\n== Dynamic Disk Header ==");
        println!(
            "Confirmed that sig is {:?}",
            String::from_utf8_lossy(vhd_dyn_disk_header.signature())
        );
        println!(
            "Block table offset: {}",
            vhd_dyn_disk_header.block_table_offset()
        );
        println!(
            "Number of blocks  : {}",
            vhd_dyn_disk_header.max_block_entries()
        );
        println!("Block size        : {}", vhd_dyn_disk_header.block_size());
    } else {
        println!("Found {:?} instead of cxsparse", &dyn_disk_header[0..8]);
        return Ok(());
    }

    Ok(())
}
