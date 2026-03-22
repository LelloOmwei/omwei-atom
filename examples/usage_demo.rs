//! Comprehensive 32BSA Usage Demonstration
//!
//! This example shows advanced usage patterns including:
//! - Batch processing of atoms
//! - Post-quantum cryptography (PQC) signing
//! - Semantic validation and trust scoring
//! - Hardware-friendly serialization

use bsa_types::{
    SemanticAtom, PREDICATE_ANOMALY, PREDICATE_CRITICAL, PREDICATE_NORMAL, PREDICATE_WARNING,
    TAG_HYDROLOGICAL, TAG_METEOROLOGICAL, TELEMETRY_HUMIDITY_PERCENT, TELEMETRY_PRECIPITATION_MM,
    TELEMETRY_TEMPERATURE_C, TELEMETRY_WATER_LEVEL_MM, TRUST_ANOMALY, TRUST_RAW, TRUST_VERIFIED,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("32BSA Comprehensive Usage Demo");
    println!("===============================");

    let timestamp = 1640995200000000u64; // 2022-01-01 00:00:00 UTC

    // -----------------------------------------------------------------
    // 1. Basic Atom Creation and Validation
    // -----------------------------------------------------------------
    println!("\n1. Basic Atom Creation");
    println!("--------------------");

    let water_atom = SemanticAtom::new(
        0x00001022, // EA Station entity
        TELEMETRY_WATER_LEVEL_MM,
        PREDICATE_NORMAL,
        12345, // 123.45m water level
        timestamp,
        TAG_HYDROLOGICAL,
        TRUST_VERIFIED,
    );

    println!("Created water atom: {:.2}mm", water_atom.get_value());
    println!("Entity ID: 0x{:08X}", water_atom.entity_id);
    println!("Trust level: {}", water_atom.trust_level());
    println!("Is alert: {}", water_atom.is_critical_or_warning());

    // -----------------------------------------------------------------
    // 2. Batch Processing Example
    // -----------------------------------------------------------------
    println!("\n2. Batch Processing");
    println!("-------------------");

    let mut atoms = Vec::new();

    // Temperature readings
    for i in 0..5 {
        let temp_atom = SemanticAtom::new(
            0x00004001, // Local temp sensor
            TELEMETRY_TEMPERATURE_C,
            PREDICATE_NORMAL,
            2000 + (i * 50),                   // 20.00°C to 22.00°C
            timestamp + (i as u64 * 60000000), // 1 minute intervals
            TAG_METEOROLOGICAL,
            TRUST_VERIFIED,
        );
        atoms.push(temp_atom);
    }

    // Humidity readings
    for i in 0..3 {
        let humidity_atom = SemanticAtom::new(
            0x00004002, // Local humidity sensor
            TELEMETRY_HUMIDITY_PERCENT,
            if i == 1 {
                PREDICATE_WARNING
            } else {
                PREDICATE_NORMAL
            },
            4500 + (i * 200),                  // 45.0% to 49.0%
            timestamp + (i as u64 * 30000000), // 30 second intervals
            TAG_METEOROLOGICAL,
            TRUST_VERIFIED,
        );
        atoms.push(humidity_atom);
    }

    println!("Created batch of {} atoms", atoms.len());

    // -----------------------------------------------------------------
    // 3. Serialization and Deserialization
    // -----------------------------------------------------------------
    println!("\n3. Serialization Performance");
    println!("-------------------------");

    let start = std::time::Instant::now();
    let mut serialized_atoms = Vec::new();

    for atom in &atoms {
        let bytes = atom.to_bytes();
        serialized_atoms.push(bytes);
    }

    let serialize_time = start.elapsed();
    println!("Serialized {} atoms in {:?}", atoms.len(), serialize_time);
    println!(
        "Average per atom: {:?}",
        serialize_time / atoms.len() as u32
    );

    // Test deserialization
    let start = std::time::Instant::now();
    let mut deserialized_atoms = Vec::new();

    for bytes in &serialized_atoms {
        match SemanticAtom::from_bytes(bytes) {
            Ok(atom) => deserialized_atoms.push(atom),
            Err(e) => println!("Deserialization error: {}", e),
        }
    }

    let deserialize_time = start.elapsed();
    println!(
        "Deserialized {} atoms in {:?}",
        deserialized_atoms.len(),
        deserialize_time
    );
    println!(
        "Average per atom: {:?}",
        deserialize_time / deserialized_atoms.len() as u32
    );

    // -----------------------------------------------------------------
    // 4. Post-Quantum Cryptography (if enabled)
    // -----------------------------------------------------------------
    #[cfg(feature = "pqc")]
    {
        println!("\n4. Post-Quantum Cryptography");
        println!("----------------------------");

        let atom_with_pqc = SemanticAtom::new(
            0x00001022,
            TELEMETRY_WATER_LEVEL_MM,
            PREDICATE_CRITICAL,
            15000, // 150.00m (critical level)
            timestamp,
            TAG_HYDROLOGICAL,
            TRUST_VERIFIED,
        );

        let hash = atom_with_pqc.compute_hash();
        println!("PQC hash: {:02X?}", hash);

        // Simulate setting PQC anchor
        let mut secure_atom = atom_with_pqc;
        secure_atom.pqc_anchor = 0x12345678;

        println!("Secure atom PQC anchor: 0x{:08X}", secure_atom.pqc_anchor);
    }

    #[cfg(not(feature = "pqc"))]
    {
        println!("\n4. Post-Quantum Cryptography");
        println!("----------------------------");
        println!("PQC feature not enabled. Run with: cargo run --features pqc");
    }

    // -----------------------------------------------------------------
    // 5. Semantic Analysis
    // -----------------------------------------------------------------
    println!("\n5. Semantic Analysis");
    println!("-------------------");

    let mut critical_count = 0;
    let mut warning_count = 0;
    let mut normal_count = 0;

    for atom in &deserialized_atoms {
        match atom.predicate_id {
            PREDICATE_CRITICAL => critical_count += 1,
            PREDICATE_WARNING => warning_count += 1,
            PREDICATE_NORMAL => normal_count += 1,
            _ => {} // Other predicates
        }
    }

    println!("Semantic analysis results:");
    println!("  Critical atoms: {}", critical_count);
    println!("  Warning atoms: {}", warning_count);
    println!("  Normal atoms: {}", normal_count);

    // -----------------------------------------------------------------
    // 6. Trust Level Analysis
    // -----------------------------------------------------------------
    println!("\n6. Trust Level Analysis");
    println!("---------------------");

    let mut trust_distribution = std::collections::HashMap::new();

    for atom in &deserialized_atoms {
        *trust_distribution.entry(atom.trust_level()).or_insert(0) += 1;
    }

    for (trust_level, count) in &trust_distribution {
        let trust_name = match *trust_level {
            TRUST_VERIFIED => "Verified",
            TRUST_RAW => "Raw",
            TRUST_ANOMALY => "Anomaly",
            _ => "Other",
        };
        println!("  {}: {} atoms", trust_name, count);
    }

    // -----------------------------------------------------------------
    // 7. Performance Summary
    // -----------------------------------------------------------------
    println!("\n7. Performance Summary");
    println!("---------------------");

    let total_memory = atoms.len() * std::mem::size_of::<SemanticAtom>();
    println!(
        "Memory usage: {} bytes ({} atoms × {} bytes)",
        total_memory,
        atoms.len(),
        std::mem::size_of::<SemanticAtom>()
    );

    println!(
        "Serialization throughput: {:.2} atoms/second",
        atoms.len() as f64 / serialize_time.as_secs_f64()
    );
    println!(
        "Deserialization throughput: {:.2} atoms/second",
        deserialized_atoms.len() as f64 / deserialize_time.as_secs_f64()
    );

    println!("\n✅ Demo completed successfully!");
    println!("The 32BSA standard provides efficient, deterministic,");
    println!("hardware-compatible semantic data exchange for Industry 4.0.");

    Ok(())
}
