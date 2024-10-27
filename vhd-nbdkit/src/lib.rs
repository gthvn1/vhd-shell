//! ```no_run
//! # use nbdkit::*;
//! #[derive(Default)]
//! struct MyPlugin {
//!     // ...
//!     # _not_used: i32,
//! }
//! impl Server for MyPlugin {
//!     fn get_size(&self) -> Result<i64> {
//!         // ...
//!         # Ok(0)
//!     }
//!
//!     fn name() -> &'static str {
//!         "my_plugin"
//!     }
//!
//!     fn open(_readonly: bool) -> Box<dyn Server> {
//!         Box::new(MyPlugin::default())
//!     }
//!
//!     fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<()> {
//!         // ...
//!         # Ok(())
//!     }
//! }
//! plugin!(MyPlugin {});
//! # fn main() {}
//! ```
mod vhdfile;
