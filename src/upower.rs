use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower",
    default_service = "org.freedesktop.UPower",
    assume_defaults = true
)]
pub trait UPower {
    /// EnumerateDevices method
    fn enumerate_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// DeviceAdded signal
    #[zbus(signal)]
    fn device_added(&self, device: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
}
