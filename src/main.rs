use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::{self, Read};

// VHD Specifications:
// https://github.com/libyal/libvhdi/blob/main/documentation/Virtual%20Hard%20Disk%20(VHD)%20image%20format.asciidoc

#[repr(C)]
#[derive(Debug)]
struct VhdFooter {
    signature: [u8; 8],     // "conectix"
    features: u32,          // Features
    version: u32,           // Format version (major and minor)
    next_offset: u64,       // Next offset
    modification_time: u32, // Modification time
    creator_app: u32,       // Creator application
    creator_version: u32,   // Creator version
    creator_os: u32,        // Creator (host) operating system
    disk_size: u64,         // Disk size
    data_size: u64,         // Data size
    disk_geometry: u32,     // Disk geometry
    disk_type: u32,         // Disk type
    checksum: u32,          // Checksum
    identifier: [u8; 16],   // Identifier (contains big-endian GUID)
    saved_state: u8,        // Saved state flag
    reserved: [u8; 427],    // Reserved (empty values)
}

fn parse_vhd_footer(footer_bytes: &[u8]) -> Result<VhdFooter, &'static str> {
    // Ensure the input slice has enough bytes
    if footer_bytes.len() < std::mem::size_of::<VhdFooter>() {
        return Err("Not enough bytes to read VHD footer");
    }

    let mut footer = VhdFooter {
        signature: [0; 8],
        features: 0,
        version: 0,
        next_offset: 0,
        modification_time: 0,
        creator_app: 0,
        creator_version: 0,
        creator_os: 0,
        disk_size: 0,
        data_size: 0,
        disk_geometry: 0,
        disk_type: 0,
        checksum: 0,
        identifier: [0; 16],
        saved_state: 0,
        reserved: [0; 427],
    };

    // Copy signature
    footer.signature.copy_from_slice(&footer_bytes[0..8]);

    // Read u32 fields
    footer.features = BigEndian::read_u32(&footer_bytes[8..12]);
    footer.version = BigEndian::read_u32(&footer_bytes[12..16]);
    footer.next_offset = BigEndian::read_u64(&footer_bytes[16..24]);
    footer.modification_time = BigEndian::read_u32(&footer_bytes[24..28]);
    footer.creator_app = BigEndian::read_u32(&footer_bytes[28..32]);
    footer.creator_version = BigEndian::read_u32(&footer_bytes[32..36]);
    footer.creator_os = BigEndian::read_u32(&footer_bytes[36..40]);
    footer.disk_size = BigEndian::read_u64(&footer_bytes[40..48]);
    footer.data_size = BigEndian::read_u64(&footer_bytes[48..56]);
    footer.disk_geometry = BigEndian::read_u32(&footer_bytes[56..60]);
    footer.disk_type = BigEndian::read_u32(&footer_bytes[60..64]);
    footer.checksum = BigEndian::read_u32(&footer_bytes[64..68]);
    footer.identifier.copy_from_slice(&footer_bytes[68..84]);
    footer.saved_state = footer_bytes[84];

    // Copy the reserved bytes
    footer.reserved.copy_from_slice(&footer_bytes[85..512]);

    Ok(footer)
}

fn main() -> io::Result<()> {
    // Open a VHD file in binary mode
    let file = File::open("test.vhd")?;

    // Read the VHD Footer into a buffer
    // There is a copy at the beginning of the file and the FOOTER is 512 bytes
    let mut footer = Vec::new();
    file.take(512).read_to_end(&mut footer)?;

    // Before parsing the footer ensure that at least signature is correct.
    if &footer[0..8] == "conectix".as_bytes() {
        let vhd_footer = parse_vhd_footer(&footer).unwrap();
        println!(
            "Confirmed that sig is {:?}",
            String::from_utf8_lossy(&vhd_footer.signature)
        );
        println!("next offset: {}", vhd_footer.next_offset);
    } else {
        println!("Found {:?} instead of conectix", &footer[0..8]);
    }

    Ok(())
}
