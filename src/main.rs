mod upower;

use async_std::{prelude::*, task};
use glob::glob;
use palette::{rgb::Rgb, Hsv, IntoColor};
use std::{error::Error, fs::write};
use upower::{UPowerDeviceProxy, UPowerProxy};
use zbus::{zvariant::ObjectPath, Connection};

const BRIGHTNESS: f64 = 0.2;

fn update_color(native_path: &str, percentage: f64) -> Result<(), Box<dyn Error>> {
    let color = Hsv::new(percentage / 100.0 * 120.0, 1.0, BRIGHTNESS);
    let (r, g, b) = IntoColor::<Rgb<_, _>>::into_color(color)
        .into_format::<u8>()
        .into_components();

    println!("Updating {native_path} with {percentage}% to #{r:02x}{g:02x}{b:02x}",);

    for (channel, value) in [("red", r), ("green", g), ("blue", b)] {
        let led_path = glob(&format!(
            "/sys/class/power_supply/{native_path}/device/leds/*:{channel}/brightness",
        ))?
        .next()
        .ok_or(format!("Missing LED for color {channel}"))??;
        write(led_path, value.to_string())?;
    }

    Ok(())
}

async fn handle_device(
    connection: &Connection,
    path: ObjectPath<'_>,
) -> Result<(), Box<dyn Error>> {
    if !path.contains("gaming_input_sony_controller_battery") {
        return Ok(());
    }
    let device = UPowerDeviceProxy::builder(connection)
        .path(&path.to_owned())?
        .build()
        .await?;

    let native_path = device.native_path().await?;

    update_color(&native_path, device.percentage().await?)?;

    let mut percentage_changed_stream = device.receive_percentage_changed().await;
    task::spawn(async move {
        while let Some(signal) = percentage_changed_stream.next().await {
            let percentage = signal.get().await.unwrap();
            if let Err(e) = update_color(&native_path, percentage) {
                panic!("Error updating the color: {}", e)
            }
        }
    });

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;
    let upower = UPowerProxy::new(&connection).await?;

    for path in upower.enumerate_devices().await? {
        handle_device(&connection, path.into_inner()).await?;
    }

    let mut device_added_stream = upower.receive_device_added().await?;
    while let Some(signal) = device_added_stream.next().await {
        let args = signal.args()?;
        handle_device(&connection, args.path).await?;
    }

    Ok(())
}
