use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

use qcow_lib::qcow::QcowHeader;
use vhd_lib::vhd::{self, DynamicDiskHeader, VhdFooter};

fn reading_vhd_file(filename: &str) -> io::Result<()> {
    // Open a VHD file in binary mode
    let mut file = File::open(filename)?;

    // Read the VHD Footer into a buffer
    // There is a copy at the beginning of the file and the FOOTER is 512 bytes
    let mut footer = [0u8; 512];
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut footer)?;

    let vhd_footer = VhdFooter::parse(&footer).unwrap();
    if vhd_footer.signature() != "conectix".as_bytes() {
        // If we don't find the signature no need to read the dynamic disk header
        println!("Found {:?} instead of conectix", &footer[0..8]);
        return Ok(());
    }
    println!("## VHD FOOTER");
    println!("next offset: {}", vhd_footer.next_offset());
    println!("Disk size  : {}", vhd_footer.disk_size());
    println!("Data size  : {}", vhd_footer.data_size());
    println!(
        "Disk geometry: Cylinders {}/ Heads {}/ Sectors {}",
        vhd_footer.disk_cylinders(),
        vhd_footer.disk_heads(),
        vhd_footer.disk_sectors()
    );

    // Read the dynamic disk header
    let mut dyn_disk_header = [0u8; 1024];
    file.seek(SeekFrom::Start(512))?;
    file.read_exact(&mut dyn_disk_header)?;

    let vhd_dyn_disk_header = DynamicDiskHeader::parse(&dyn_disk_header).unwrap();

    // Before parsing the dynamic disk header ensure that signature is correct.
    if vhd_dyn_disk_header.signature() != "cxsparse".as_bytes() {
        println!("Found {:?} instead of cxsparse", &dyn_disk_header[0..8]);
        return Ok(());
    }

    let bat_offset = vhd_dyn_disk_header.block_table_offset();
    let num_of_block = vhd_dyn_disk_header.max_block_entries() as usize;
    let block_size = vhd_dyn_disk_header.block_size();

    println!("\n## Dynamic Disk Header");
    println!("Block table offset: {}", bat_offset);
    println!("Number of blocks  : {}", num_of_block);
    println!("Block size        : {}", block_size);

    // Get the block entries
    println!("\n## BAT info");
    let bat_entries = vhd::read_bat_entries(&mut file, bat_offset, num_of_block)?;
    for (idx, entry) in bat_entries.iter().enumerate() {
        print!("Block#{:04} -> 0x{:08x} : ", idx, entry,);
        if *entry == 0xFFFF_FFFF {
            println!("block is not allocated")
        } else {
            println!(
                "bitmap [0x{:08x}-0x{:08x}], data [0x{:08x}-0x{:08x}]",
                entry * 512,
                (entry * 512 + 512) - 1,
                entry * 512 + 512,
                entry * 512 + block_size - 1,
            );
        }
    }

    Ok(())
}

fn reading_qcow_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let header = QcowHeader::from_reader(&mut reader)?;

    println!("{:#?}", header);
    Ok(())
}

fn main() -> io::Result<()> {
    println!("# Reading info from test.vhd file");
    reading_vhd_file("test.vhd")?;

    println!("\n# Reading info from test.qcow2 file");
    reading_qcow_file("test.qcow2")?;

    Ok(())
}
