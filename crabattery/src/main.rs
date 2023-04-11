use menu_rs::*;
use zbus::{dbus_proxy, Connection, Result};

enum Mode {
    Lifespan,
    Balanced,
    Full,
}

#[dbus_proxy(
    interface = "ar.claaj.Crabattery.Limiter",
    default_service = "ar.claaj.Crabattery",
    default_path = "/ar/claaj/Crabattery"
)]
trait Limiter {
    async fn set_battery_limit(&self, limit: u8, mode: &str) -> Result<String>;
}

fn main() {
    let menu = Menu::new(vec![
        MenuOption::new("Maximum Lifespan Mode", || connection(Mode::Lifespan))
            .hint("Charging limit at 60%."),
        MenuOption::new("Balanced Mode", || connection(Mode::Balanced))
            .hint("Charging limit at 80%."),
        MenuOption::new("Full Capacity Mode", || connection(Mode::Full)).hint("Full charge."),
    ]);

    menu.title("Crabattery - Charging Limiter🔋🔌").show();
}

fn connection(mode: Mode) {
    let (limit, mode) = match mode {
        Mode::Lifespan => (60, "Maximum Lifespan"),
        Mode::Balanced => (80, "Balanced"),
        Mode::Full => (100, "Full capacity"),
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _crabattery_connection = Connection::system().await.unwrap();
        let crabattery_proxy = LimiterProxy::new(&_crabattery_connection).await.unwrap();
        let crabattery_reply = crabattery_proxy.set_battery_limit(limit, mode).await.unwrap();
        println!("{}", crabattery_reply);
    });
}
