use crate::building::Building;
use crate::heat_recovery_plant::HeatRecoveryPlant;
use crate::location::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Context {
    pub building: Building,

    pub location: Location,

    // θint,H,set
    pub indoor_temp_heating: f32,

    // θint,C,set
    pub indoor_temp_cooling: f32,

    pub heat_recovery_plant: HeatRecoveryPlant,
}
