use battery::{Manager, State};

#[derive(Copy, Clone)]
pub struct BatteryInformation {
    pub percentages: u8,
    pub is_charging: bool,
}

impl BatteryInformation {
    pub fn new() -> Result<Self, ()> {
        let mut batteries = Manager::new().and_then(|m| m.batteries()).map_err(|_| ())?;
        let battery = batteries.next().ok_or(())?.map_err(|_| ())?;

        let percentages = battery
            .state_of_charge()
            .get::<battery::units::ratio::percent>() as u8;

        let is_charging = battery.state() != State::Discharging;

        Ok(Self {
            percentages,
            is_charging,
        })
    }
}
