use zbus::zvariant::{ObjectPath, OwnedObjectPath};
use zbus::{dbus_proxy, Result};

#[dbus_proxy(
    default_service = "org.freedesktop.UPower",
    interface = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
trait UPower {
    fn enumerate_devices(&self) -> Result<Vec<OwnedObjectPath>>;

    #[dbus_proxy(signal)]
    fn device_added(&self, path: ObjectPath<'_>) -> Result<()>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.UPower",
    interface = "org.freedesktop.UPower.Device"
)]
trait UPowerDevice {
    #[dbus_proxy(property)]
    fn native_path(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn percentage(&self) -> Result<f64>;
}
