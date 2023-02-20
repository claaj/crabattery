use std::{error::Error, future::pending};
use tokio::{time::{sleep, Duration}, process::Command};
use zbus::{dbus_interface, ConnectionBuilder};

struct Limiter;

#[dbus_interface(name = "ar.claaj.Rusttery.Limiter")]
impl Limiter {
    async fn set_battery_limit(&self, limit: u8, mode: &str) -> String {

        let command = format!("echo {} | tee /sys/class/power_supply/BAT0/charge_control_end_threshold", limit);
        let output = Command::new("sh").arg("-c").arg(&command).output().await;

        sleep(Duration::from_millis(100)).await;

        match output {
            Ok(_status) => {
                return format!("✅ {} mode activated! Limit set to: {}%.", mode, limit);
            },
            Err(err) => {
                return format!("❌ Failed to activate {} mode. Err: {}", mode, err);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _connection = ConnectionBuilder::system()?
        .name("ar.claaj.Rusttery")?
        .serve_at("/ar/claaj/Rusttery", Limiter)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
