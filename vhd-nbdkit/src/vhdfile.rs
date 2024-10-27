use lazy_static::lazy_static;
use nbdkit::*;
use std::sync::Mutex;

// Use RAM disk for testing.
lazy_static! {
    static ref DISK: Mutex<Vec<u8>> = Mutex::new(vec![0; 100 * 1024 * 1024]);
}

#[derive(Default)]
struct VhdFile {
    _not_used: i32,
}

impl Server for VhdFile {
    fn get_size(&self) -> Result<i64> {
        Ok(DISK.lock().unwrap().len() as i64)
    }

    fn name() -> &'static str {
        "vhdfile"
    }

    fn open(_readonly: bool) -> Result<Box<dyn Server>> {
        Ok(Box::<VhdFile>::default())
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<()> {
        let disk = DISK.lock().unwrap();
        let ofs = offset as usize;
        let end = ofs + buf.len();
        eprintln!("Read at {:08x} ends {:08x}", ofs, end);
        buf.copy_from_slice(&disk[ofs..end]);
        Ok(())
    }

    fn thread_model() -> Result<ThreadModel>
    where
        Self: Sized,
    {
        Ok(ThreadModel::Parallel)
    }

    fn write_at(&self, buf: &[u8], offset: u64, _flags: Flags) -> Result<()> {
        let mut disk = DISK.lock().unwrap();
        let ofs = offset as usize;
        let end = ofs + buf.len();
        eprintln!("Write at {:08x} ends {:08x}", ofs, end);
        disk[ofs..end].copy_from_slice(buf);
        Ok(())
    }
}

plugin!(VhdFile {
    thread_model,
    write_at
});
