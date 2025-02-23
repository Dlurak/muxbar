use battery::{Manager, State};

#[derive(Copy, Clone)]
pub struct BatteryInformation {
    pub percentages: u8,
    pub is_charging: bool,
}

impl BatteryInformation {
    pub fn new() -> Result<Option<Self>, ()> {
        let mut batteries = Manager::new().and_then(|m| m.batteries()).map_err(|_| ())?;
        let battery = match batteries.next() {
            Some(Ok(bat)) => bat,
            Some(Err(_)) => return Err(()),
            None => return Ok(None),
        };

        let percentages = battery
            .state_of_charge()
            .get::<battery::units::ratio::percent>() as u8;

        let is_charging = battery.state() != State::Discharging;

        Ok(Some(Self {
            percentages,
            is_charging,
        }))
    }
}
