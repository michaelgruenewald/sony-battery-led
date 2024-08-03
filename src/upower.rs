use zbus::zvariant::{ObjectPath, OwnedObjectPath};
use zbus::{proxy, Result};

#[proxy(
    default_service = "org.freedesktop.UPower",
    interface = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
trait UPower {
    fn enumerate_devices(&self) -> Result<Vec<OwnedObjectPath>>;

    #[zbus(signal)]
    fn device_added(&self, path: ObjectPath<'_>) -> Result<()>;
}

#[proxy(
    default_service = "org.freedesktop.UPower",
    interface = "org.freedesktop.UPower.Device"
)]
trait UPowerDevice {
    #[zbus(property)]
    fn native_path(&self) -> Result<String>;

    #[zbus(property)]
    fn percentage(&self) -> Result<f64>;
}
