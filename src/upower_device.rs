use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    assume_defaults = true
)]
pub trait UPowerDevice {
    /// NativePath property
    #[zbus(property)]
    fn native_path(&self) -> zbus::Result<String>;

    /// Percentage property
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;
}
