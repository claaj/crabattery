use std::{error::Error, future::pending, process::Output, path::Path};
use tokio::{time::{sleep, Duration},
            process::Command,
            fs::{read_to_string, write}};
use zbus::{dbus_interface, ConnectionBuilder};

const LIMIT_PATH :&str = "/etc/rusttery";
const LIMIT_FILE :&str = "rusttery.limit";

struct Limiter;

#[dbus_interface(name = "ar.claaj.Rusttery.Limiter")]
impl Limiter {
    async fn set_battery_limit(&self, limit: u8, mode: &str) -> String {
        let write_result = write_limit_to_file(limit).await;

        let output = sh_change_limit(limit).await;

        sleep(Duration::from_millis(100)).await;

        match output {
            Ok(status) if status.status.success() && write_result.is_ok() => {
                format!("✅ {} mode activated! Limit set to: {}%.", mode, limit)
            },
            Ok(status) if status.status.success() && write_result.is_err() => {
                format!("✅ {} mode activated! Limit set to: {}%.\n🚨 Limit will reset after reboot.", mode, limit)
            },
            Err(err) => {
                format!("❌ Failed to activate {} mode. Err: {}", mode, err)
            },
            _ => {
                format!("❌ Failed to activate {} mode.", mode)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _rusttery_connection = ConnectionBuilder::system()?
        .name("ar.claaj.Rusttery")?
        .serve_at("/ar/claaj/Rusttery", Limiter)?
        .build()
        .await?;

    let limit = check_existing_limit().await;
    sh_change_limit(limit).await?;

    pending::<()>().await;

    Ok(())
}

async fn sh_change_limit(limit :u8) -> Result<Output, std::io::Error> {
    let command = format!("echo {} | tee /sys/class/power_supply/BAT?/charge_control_end_threshold", limit);
    Command::new("sh").arg("-c").arg(&command).output().await
}

async fn check_existing_limit() -> u8 {
    let path = Path::new(LIMIT_PATH).join(LIMIT_FILE);

    let limit_string = read_to_string(path).await;
    match limit_string {
        Ok(string) => {
            let number = string.parse::<u8>().unwrap_or(100);
            number
        },
        Err(_err) => {
            100
        }
    }
}

async fn write_limit_to_file(limit :u8) -> Result<(), std::io::Error> {
    let path = Path::new(LIMIT_PATH).join(LIMIT_FILE);
    let limit_string = format!("{}", limit);
    write(path, limit_string.as_bytes()).await
}
