use dbus::{arg::AppendAll, blocking::Connection, Error};
use ksni::Icon;
use std::fmt;
use std::{fmt::Display, str::FromStr, time::Duration};

use crate::utils::icon_from_bytes;

static ECO_ICON: &[u8; 312] = include_bytes!("../../icons/leaf.png");
static BALANCE_ICON: &[u8; 549] = include_bytes!("../../icons/balance.png");
static SPORT_ICON: &[u8; 623] = include_bytes!("../../icons/performance.png");

#[derive(Debug, Clone, Copy)]
pub enum PowerMode {
    Eco,
    Comfort,
    Sport,
}

impl PowerMode {
    pub fn icon(&self) -> Icon {
        match self {
            PowerMode::Eco => icon_from_bytes(ECO_ICON),
            PowerMode::Sport => icon_from_bytes(SPORT_ICON),
            PowerMode::Comfort => icon_from_bytes(BALANCE_ICON),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePowerModeError;

impl FromStr for PowerMode {
    type Err = ParsePowerModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "eco" => Ok(PowerMode::Eco),
            "comfort" => Ok(PowerMode::Comfort),
            "sport" => Ok(PowerMode::Sport),
            _ => Err(ParsePowerModeError),
        }
    }
}

impl Display for PowerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_val = match self {
            PowerMode::Eco => "eco",
            PowerMode::Sport => "sport",
            PowerMode::Comfort => "comfort",
        };
        write!(f, "{}", str_val)
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

impl From<PowerMode> for usize {
    fn from(value: PowerMode) -> Self {
        match value {
            PowerMode::Eco => 0,
            PowerMode::Comfort => 1,
            PowerMode::Sport => 2,
        }
    }
}

impl From<usize> for PowerMode {
    fn from(value: usize) -> Self {
        match value {
            0 => PowerMode::Eco,
            1 => PowerMode::Comfort,
            2 => PowerMode::Sport,
            _ => PowerMode::Comfort,
        }
    }
}

pub struct PowerModeController {
    dbus_conn: Connection,
}

impl PowerModeController {
    pub fn new() -> Self {
        let conn = Connection::new_system().unwrap();

        PowerModeController { dbus_conn: conn }
    }

    fn run_dbus_method<A>(&self, method_name: &str, args: A) -> Result<Option<String>, Error>
    where
        A: AppendAll,
    {
        let proxy = &self.dbus_conn.with_proxy(
            "org.msi_ec_dbus",
            "/power_control",
            Duration::from_millis(5000),
        );

        let (result,): (String,) = proxy.method_call("org.msi_ec_dbus", method_name, args)?;

        Ok(Some(result))
    }

    pub fn current_power_mode(&self) -> Result<PowerMode, Error> {
        let res = self.run_dbus_method("GetCurrentPowerMode", ())?;
        if let Some(current_power_mode) =
            res.and_then(|res_val: String| PowerMode::from_str(res_val.as_str()).ok())
        {
            return Ok(current_power_mode);
        };
        Err(Error::new_failed(
            "Error with retrieving current power mode",
        ))
    }

    pub fn set_power_mode(&self, power_mode: PowerMode) -> Result<(), Error> {
        let power_mode_str: &str = power_mode.into();
        self.run_dbus_method("SetPowerMode", (power_mode_str,))?;

        Ok(())
    }
}
