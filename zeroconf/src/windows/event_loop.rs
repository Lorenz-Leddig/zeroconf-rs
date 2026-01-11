//! Event loop for running a `MdnsBrowser`.

use std::time::Duration;

use crate::Result;
use crate::event_loop::TEventLoop;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
pub struct WindowsEventLoop;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
impl TEventLoop for WindowsEventLoop {
    fn poll(&self, _timeout: Duration) -> Result<()> {
        todo!()
    }
}
