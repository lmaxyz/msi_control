use std::time::Duration;
use dbus::{blocking::Connection, Error, arg::AppendAll};


#[derive(Debug, Clone, Copy)]
pub enum PowerMode {
    Eco,
    Comfort,
    Sport
}

impl PowerMode {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "eco" => Some(PowerMode::Eco),
            "comfort" => Some(PowerMode::Comfort),
            "sport" => Some(PowerMode::Sport),
            _ => None
        }
    }
}

impl From<PowerMode> for &str {
    fn from(value: PowerMode) -> Self {
        match value {
            PowerMode::Eco => "eco",
            PowerMode::Comfort => "comfort",
            PowerMode::Sport => "sport",
        }
    }
}

impl From<PowerMode> for String {
    fn from(value: PowerMode) -> Self {
        match value {
            PowerMode::Eco => "eco".to_string(),
            PowerMode::Comfort => "comfort".to_string(),
            PowerMode::Sport => "sport".to_string(),
        }
    }
}

impl From<PowerMode> for usize {
    fn from(value: PowerMode) -> Self {
        match value {
            PowerMode::Eco => 0,
            PowerMode::Comfort => 1,
            PowerMode::Sport => 2
        }
    }
}

impl From<usize> for PowerMode {
    fn from(value: usize) -> Self {
        match value {
            0 => PowerMode::Eco,
            1 => PowerMode::Comfort,
            _ => PowerMode::Sport,
        }
    }
}


pub struct PowerModeController {
    dbus_conn: Connection
}

impl PowerModeController {
    pub fn new() -> Self {
        let conn = Connection::new_system().unwrap();

        PowerModeController{
            dbus_conn: conn
        }
    }

    fn run_dbus_method<A>(&self, method_name: &str, args: A) -> Result<Option<String>, Error> 
    where
        A: AppendAll
    {
        let proxy = &self.dbus_conn.with_proxy("org.msi_ec_backend", "/power_control", Duration::from_millis(5000));
        
        let (result,): (String,) = proxy.method_call("org.msi_ec_backend", method_name, args)?;

        Ok(Some(result))
    }

    pub fn get_current_power_mode(&self) -> Result<PowerMode, Error> {
        let res = self.run_dbus_method("GetCurrentPowerMode", ())?;
        if let Some(current_power_mode) = res.and_then(|res_val: String| { PowerMode::from_str(res_val.as_str()) }) {
            return Ok(current_power_mode)
        };
        Err(Error::new_failed("Error with retriving current power mode"))
    }

    pub fn set_power_mode(&self, power_mode: PowerMode) -> Result<(), Error> {
        let power_mode_str: &str = power_mode.into();
        self.run_dbus_method("SetPowerMode", (power_mode_str,))?;
        
        Ok(())
    }
}
