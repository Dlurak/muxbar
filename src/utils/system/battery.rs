use battery::{Manager, State};

pub fn battery_status() -> Result<u8, ()> {
    let mut batteries = Manager::new().and_then(|m| m.batteries()).map_err(|_| ())?;
    let battery = batteries.next().ok_or(())?.map_err(|_| ())?;

    let percentages = battery
        .state_of_charge()
        .get::<battery::units::ratio::percent>() as u8;

    Ok(percentages)
}

pub fn battery_charging() -> Result<bool, ()> {
    let mut batteries = Manager::new().and_then(|m| m.batteries()).map_err(|_| ())?;
    let battery = batteries.next().ok_or(())?.map_err(|_| ())?;

    Ok(battery.state() == State::Charging)
}
