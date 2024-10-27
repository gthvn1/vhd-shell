use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

// VHD Specifications:
// https://github.com/libyal/libvhdi/blob/main/documentation/Virtual%20Hard%20Disk%20(VHD)%20image%20format.asciidoc

#[repr(C)]
#[derive(Debug)]
pub struct VhdFooter {
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

impl VhdFooter {
    pub fn parse(footer_bytes: &[u8]) -> Result<VhdFooter, &'static str> {
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

    pub fn signature(&self) -> &[u8; 8] {
        &self.signature
    }

    pub fn next_offset(&self) -> u64 {
        self.next_offset
    }

    pub fn disk_size(&self) -> u64 {
        self.disk_size
    }

    pub fn data_size(&self) -> u64 {
        self.data_size
    }

    pub fn disk_cylinders(&self) -> u32 {
        (self.disk_geometry & 0xFFFF_0000) >> 16
    }

    pub fn disk_heads(&self) -> u32 {
        (self.disk_geometry & 0x0000_FF00) >> 8
    }

    pub fn disk_sectors(&self) -> u32 {
        self.disk_geometry & 0x0000_00FF
    }
}

#[derive(Debug)]
pub struct DynamicDiskHeader {
    signature: [u8; 8],                    // "cxsparse"
    next_offset: u64,                      // Next offset (8 bytes)
    block_table_offset: u64,               // Block table offset (8 bytes)
    format_version: u32,                   // Format version (4 bytes)
    max_block_entries: u32,                // Number of blocks (4 bytes)
    block_size: u32,                       // Block size (4 bytes)
    checksum: u32,                         // Checksum (4 bytes)
    parent_id: [u8; 16],                   // Parent identifier (16 bytes, GUID)
    parent_modification_time: u32,         // Parent modification time (4 bytes)
    reserved: u32,                         // Reserved (4 bytes)
    parent_name: [u8; 512],                // Parent name (UTF-16, 512 bytes)
    parent_locator_entries: [[u8; 8]; 24], // Array of parent locator entries (8 bytes x 24 entries)
    reserved_2: [u8; 256],                 // Reserved (256 bytes)
}

impl DynamicDiskHeader {
    pub fn parse(header_bytes: &[u8]) -> Result<DynamicDiskHeader, &'static str> {
        if header_bytes.len() < 1024 {
            return Err("Not enough bytes to read dynamic disk header");
        }

        let mut header = DynamicDiskHeader {
            signature: [0; 8],
            next_offset: 0,
            block_table_offset: 0,
            format_version: 0,
            max_block_entries: 0,
            block_size: 0,
            checksum: 0,
            parent_id: [0; 16],
            parent_modification_time: 0,
            reserved: 0,
            parent_name: [0; 512],
            parent_locator_entries: [[0; 8]; 24],
            reserved_2: [0; 256],
        };

        header.signature.copy_from_slice(&header_bytes[0..8]);
        header.next_offset = BigEndian::read_u64(&header_bytes[8..16]);
        header.block_table_offset = BigEndian::read_u64(&header_bytes[16..24]);
        header.format_version = BigEndian::read_u32(&header_bytes[24..28]);
        header.max_block_entries = BigEndian::read_u32(&header_bytes[28..32]);
        header.block_size = BigEndian::read_u32(&header_bytes[32..36]);
        header.checksum = BigEndian::read_u32(&header_bytes[36..40]);
        header.parent_id.copy_from_slice(&header_bytes[40..56]);
        header.parent_modification_time = BigEndian::read_u32(&header_bytes[56..60]);
        header.reserved = BigEndian::read_u32(&header_bytes[60..64]);
        header.parent_name.copy_from_slice(&header_bytes[64..576]);

        for i in 0..24 {
            header.parent_locator_entries[i]
                .copy_from_slice(&header_bytes[576 + i * 8..584 + i * 8]);
        }

        header.reserved_2.copy_from_slice(&header_bytes[768..1024]);

        Ok(header)
    }

    pub fn signature(&self) -> &[u8; 8] {
        &self.signature
    }

    pub fn block_table_offset(&self) -> u64 {
        self.block_table_offset
    }

    pub fn max_block_entries(&self) -> u32 {
        self.max_block_entries
    }

    pub fn block_size(&self) -> u32 {
        self.block_size
    }
}

pub fn read_bat_entries(f: &mut File, bat_offset: u64, num_entries: usize) -> io::Result<Vec<u32>> {
    f.seek(SeekFrom::Start(bat_offset))?;
    let mut bat_entries = Vec::with_capacity(num_entries);
    // Entry is 4 bytes, so read 4 bytes at a time
    let mut buffer = [0u8; 4];

    for _ in 0..num_entries {
        f.read_exact(&mut buffer)?;
        let entry = u32::from_be_bytes(buffer);
        bat_entries.push(entry);
    }

    Ok(bat_entries)
}
