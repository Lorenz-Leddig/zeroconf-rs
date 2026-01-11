//! Windows-specific ZeroConf bindings
//!
//! This module wraps the native [Windows] mDNS api.

pub mod browser;
pub mod event_loop;
pub mod service;
pub mod txt_record;
pub mod windows_util;
