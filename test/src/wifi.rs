// wifi.rs
use anyhow::Error;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration as ClientConfigurationSvc, Configuration as ConfigurationSvc};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;
use std::error::Error as StdError;
use heapless::String;

const SSID_MAX_LEN: usize = 32;
const PASSWORD_MAX_LEN: usize = 64;

fn connect(ssid: &str, password: &str) -> Result<(), Box<dyn StdError>> {
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;

    let mut client_config = ClientConfigurationSvc::default();

    client_config.ssid = String::from_str(ssid).map_err(|_| anyhow::anyhow!("SSID too long"))?;
    client_config.password = String::from_str(password).map_err(|_| anyhow::anyhow!("Password too long"))?;
    client_config.auth_method = AuthMethod::WPA2Personal;

    wifi.set_configuration(&ConfigurationSvc::Client(client_config))?;

    wifi.start()?;
    wifi.connect()?;

    while!wifi.is_connected().unwrap() {
        let config = wifi.get_configuration().unwrap();
        println!("connecting to {:?}", config);
    }

    println!("wifi connected successfully");

    Ok(())
}