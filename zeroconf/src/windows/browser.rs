//! Windows implementation for cross-platform browser

use std::any::Any;

use crate::browser::TMdnsBrowser;
use crate::{EventLoop, NetworkInterface, Result, ServiceBrowserCallback, ServiceType};

#[cfg(all(target_os = "windows", feature = "windows-native"))]
pub struct WindowsMdnsBrowser;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
impl TMdnsBrowser for WindowsMdnsBrowser {
    fn new(_service_type: ServiceType) -> Self {
        todo!()
    }

    fn set_network_interface(&mut self, _interface: NetworkInterface) {
        todo!()
    }

    fn network_interface(&self) -> NetworkInterface {
        todo!()
    }

    fn set_service_callback(&mut self, _service_callback: Box<ServiceBrowserCallback>) {
        todo!()
    }

    fn set_context(&mut self, _context: Box<dyn Any + Send + Sync>) {
        todo!()
    }

    fn context(&self) -> Option<&(dyn Any + Send + Sync)> {
        todo!()
    }

    fn browse_services(&mut self) -> Result<EventLoop> {
        todo!()
    }
}
