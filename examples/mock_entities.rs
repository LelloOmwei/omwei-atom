//! Mock Entity ID Implementations for 32BSA Examples
//!
//! This file demonstrates how to define entity IDs for a specific deployment.
//! The core library remains entity-agnostic - these are examples only.

use bsa_types::{
    SemanticAtom, PREDICATE_NORMAL, TELEMETRY_PRECIPITATION_MM, TELEMETRY_TEMPERATURE_C,
    TELEMETRY_WATER_LEVEL_MM, TRUST_VERIFIED,
};

/// Environment Agency water level station
pub const ENTITY_EA_STATION: u32 = 0x00001022;

/// OpenWeatherMap API integration
pub const ENTITY_OWM_WEATHER: u32 = 0x00002001;

/// Virtual station - Armley area
pub const VIRTUAL_STATION_ARMLEY: u32 = 0x00003001;

/// Virtual station - Crown Point area
pub const VIRTUAL_STATION_CROWN_POINT: u32 = 0x00003002;

/// Local temperature sensor
pub const ENTITY_LOCAL_TEMP_SENSOR: u32 = 0x00004001;

/// Local humidity sensor
pub const ENTITY_LOCAL_HUMIDITY_SENSOR: u32 = 0x00004002;

/// Weather station API integration
pub const ENTITY_WEATHER_API: u32 = 0x00005001;

/// River level monitoring system
pub const ENTITY_RIVER_MONITOR: u32 = 0x00006001;

/// Create EA water level atom
pub fn create_ea_water_level(value_mm: u32, timestamp_us: u64) -> SemanticAtom {
    use bsa_types::{PREDICATE_NORMAL, TELEMETRY_WATER_LEVEL_MM, TRUST_VERIFIED};

    SemanticAtom::new(
        ENTITY_EA_STATION,
        TELEMETRY_WATER_LEVEL_MM,
        PREDICATE_NORMAL,
        value_mm,
        timestamp_us,
        0x001, // Hydrological tag
        TRUST_VERIFIED,
    )
}

/// Create local temperature atom
pub fn create_local_temperature(value_mm: u32, timestamp_us: u64) -> SemanticAtom {
    use bsa_types::{PREDICATE_NORMAL, TELEMETRY_TEMPERATURE_C, TRUST_VERIFIED};

    SemanticAtom::new(
        ENTITY_LOCAL_TEMP_SENSOR,
        TELEMETRY_TEMPERATURE_C,
        PREDICATE_NORMAL,
        value_mm,
        timestamp_us,
        0x002, // Meteorological tag
        TRUST_VERIFIED,
    )
}

/// Create weather precipitation atom
pub fn create_weather_precipitation(
    value_mm: u32,
    timestamp_us: u64,
    trust_level: u8,
) -> SemanticAtom {
    use bsa_types::{PREDICATE_NORMAL, TELEMETRY_PRECIPITATION_MM};

    SemanticAtom::new(
        ENTITY_WEATHER_API,
        TELEMETRY_PRECIPITATION_MM,
        PREDICATE_NORMAL,
        value_mm,
        timestamp_us,
        0x002, // Meteorological tag
        trust_level,
    )
}

/// Get human-readable description for entity
pub fn get_entity_description(entity_id: u32) -> &'static str {
    match entity_id {
        ENTITY_EA_STATION => "Environment Agency water level station",
        ENTITY_OWM_WEATHER => "OpenWeatherMap weather API",
        VIRTUAL_STATION_ARMLEY => "Virtual station - Armley area",
        VIRTUAL_STATION_CROWN_POINT => "Virtual station - Crown Point area",
        ENTITY_LOCAL_TEMP_SENSOR => "Local temperature sensor",
        ENTITY_LOCAL_HUMIDITY_SENSOR => "Local humidity sensor",
        ENTITY_WEATHER_API => "Weather station API integration",
        ENTITY_RIVER_MONITOR => "River level monitoring system",
        _ => "Unknown entity",
    }
}

fn main() {
    use bsa_types::TRUST_VERIFIED;

    println!("32BSA Mock Entities Example");
    println!("==========================");

    let timestamp = 1640995200000000u64;

    // Create example atoms
    let water_atom = create_ea_water_level(12345, timestamp);
    let temp_atom = create_local_temperature(2345, timestamp);
    let precip_atom = create_weather_precipitation(678, timestamp, TRUST_VERIFIED);

    println!(
        "Water level atom: {:.2}mm from {}",
        water_atom.get_value(),
        get_entity_description(water_atom.entity_id)
    );

    println!(
        "Temperature atom: {:.2}°C from {}",
        temp_atom.get_value(),
        get_entity_description(temp_atom.entity_id)
    );

    println!(
        "Precipitation atom: {:.2}mm from {}",
        precip_atom.get_value(),
        get_entity_description(precip_atom.entity_id)
    );

    // Test serialization
    let bytes = water_atom.to_bytes();
    println!("Serialized water atom: {} bytes", bytes.len());

    // Test deserialization
    match SemanticAtom::from_bytes(&bytes) {
        Ok(restored) => println!("Successfully restored atom: {:.2}mm", restored.get_value()),
        Err(e) => println!("Error restoring atom: {}", e),
    }
}
