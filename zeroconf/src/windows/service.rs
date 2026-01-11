//! Windows implementation for cross-platform service.

use crate::prelude::*;
use crate::{
    EventLoop, NetworkInterface, Result, ServiceRegisteredCallback, ServiceType, TxtRecord,
};
use std::any::Any;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
pub struct WindowsMdnsService;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
impl TMdnsService for WindowsMdnsService {
    fn new(_service_type: ServiceType, _port: u16) -> Self {
        todo!()
    }

    fn set_name(&mut self, _name: &str) {
        todo!()
    }

    fn name(&self) -> Option<&str> {
        todo!()
    }

    fn set_network_interface(&mut self, _interface: NetworkInterface) {
        todo!()
    }

    fn network_interface(&self) -> NetworkInterface {
        todo!()
    }

    fn set_domain(&mut self, _domain: &str) {
        todo!()
    }

    fn domain(&self) -> Option<&str> {
        todo!()
    }

    fn set_host(&mut self, _host: &str) {
        todo!()
    }

    fn host(&self) -> Option<&str> {
        todo!()
    }

    fn set_txt_record(&mut self, _txt_record: TxtRecord) {
        todo!()
    }

    fn txt_record(&self) -> Option<&TxtRecord> {
        todo!()
    }

    fn set_registered_callback(&mut self, _registered_callback: Box<ServiceRegisteredCallback>) {
        todo!()
    }

    fn set_context(&mut self, _context: Box<dyn Any + Send + Sync>) {
        todo!()
    }

    fn context(&self) -> Option<&(dyn Any + Send + Sync)> {
        todo!()
    }

    fn register(&mut self) -> Result<EventLoop> {
        todo!()
    }
}
