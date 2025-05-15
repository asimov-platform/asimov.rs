// This is free and unencumbered software released into the public domain.

use lazy_static::lazy_static;
use local_ip_address::local_ip;
use mdns_sd::ServiceDaemon;

pub use mdns_sd::{Receiver, Result, ServiceEvent, ServiceInfo};

pub static SERVICE_TYPE: &str = "_asimov._udp.local.";

lazy_static! {
    static ref DAEMON: ServiceDaemon = ServiceDaemon::new().expect("Failed to create mDNS daemon");
}

pub fn register() -> Result<()> {
    let ip = local_ip().unwrap();
    let host_name = format!("{}.local.", ip);

    let info = ServiceInfo::new(
        SERVICE_TYPE,
        "asimov-server",
        host_name.as_str(),
        ip,
        1920,
        None,
    )?;

    DAEMON.register(info)
}

pub fn shutdown() -> Result<()> {
    DAEMON.shutdown()?;
    Ok(())
}

pub fn browse() -> Result<Receiver<ServiceEvent>> {
    DAEMON.browse(SERVICE_TYPE)
}
