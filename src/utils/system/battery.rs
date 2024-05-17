use battery::Manager;

pub fn battery_status() -> Result<u8, ()> {
    let mut batteries = Manager::new().and_then(|m| m.batteries()).map_err(|_| ())?;

    let battery = batteries.next().ok_or(())?.map_err(|_| ())?;

    let percentages = battery
        .state_of_charge()
        .get::<battery::units::ratio::percent>() as u8;

    Ok(percentages)
}
