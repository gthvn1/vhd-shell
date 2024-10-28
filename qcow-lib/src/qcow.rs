use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Read};

// Qcow specs
// https://github.com/zchee/go-qcow2/blob/master/docs/specification.md

#[repr(C)]
#[derive(Debug)]
pub struct QcowHeader {
    magic: u32,                   // Magic string "QFI\xfb"
    version: u32,                 // Version (2 or 3)
    backing_file_offset: u64,     // Offset to the backing file name
    backing_file_size: u32,       // Size of the backing file name
    cluster_bits: u32,            // Bits used for addressing within a cluster
    size: u64,                    // Virtual disk size
    crypt_method: u32,            // 0 = no encryption, 1 = AES encryption
    l1_size: u32,                 // Number of entries in the L1 table
    l1_table_offset: u64,         // Offset to the active L1 table
    refcount_table_offset: u64,   // Offset to the refcount table
    refcount_table_clusters: u32, // Number of clusters for the refcount table
    nb_snapshots: u32,            // Number of snapshots in the image
    snapshots_offset: u64,        // Offset to the snapshot table
}

impl QcowHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            magic: reader.read_u32::<BigEndian>()?,
            version: reader.read_u32::<BigEndian>()?,
            backing_file_offset: reader.read_u64::<BigEndian>()?,
            backing_file_size: reader.read_u32::<BigEndian>()?,
            cluster_bits: reader.read_u32::<BigEndian>()?,
            size: reader.read_u64::<BigEndian>()?,
            crypt_method: reader.read_u32::<BigEndian>()?,
            l1_size: reader.read_u32::<BigEndian>()?,
            l1_table_offset: reader.read_u64::<BigEndian>()?,
            refcount_table_offset: reader.read_u64::<BigEndian>()?,
            refcount_table_clusters: reader.read_u32::<BigEndian>()?,
            nb_snapshots: reader.read_u32::<BigEndian>()?,
            snapshots_offset: reader.read_u64::<BigEndian>()?,
        })
    }
}
