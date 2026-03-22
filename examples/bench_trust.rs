/*!
# Trust Determination Performance Benchmark

This example specifically benchmarks the core trust determination logic
to verify we achieve the target ~8.86ns performance.

## Usage
```bash
cargo run --example bench_trust
```
*/

use std::time::Instant;

fn main() {
    println!("🚀 OMWEI/SAMS v0.2.0 Trust Determination Benchmark");
    println!("=================================================");
    
    // Test data
    let managed_atom_id = 0x00000001u32;  // Bit 31 = 0
    let community_atom_id = 0x80000001u32; // Bit 31 = 1
    
    // Benchmark trust determination
    let iterations = 1_000_000;
    
    println!("📊 Running {} iterations...", iterations);
    
    // Benchmark Managed Space trust determination
    let start = Instant::now();
    for _ in 0..iterations {
        let _trust_level = omwei_atom::get_trust_level(managed_atom_id);
    }
    let managed_duration = start.elapsed();
    
    // Benchmark Community Space trust determination
    let start = Instant::now();
    for _ in 0..iterations {
        let _trust_level = omwei_atom::get_trust_level(community_atom_id);
    }
    let community_duration = start.elapsed();
    
    // Calculate per-operation latency
    let managed_ns_per_op = managed_duration.as_nanos() / iterations as u128;
    let community_ns_per_op = community_duration.as_nanos() / iterations as u128;
    
    println!("\n📈 Results");
    println!("==========");
    println!("🔹 Managed Space: {} ns per operation", managed_ns_per_op);
    println!("🔹 Community Space: {} ns per operation", community_ns_per_op);
    
    // Target performance
    let target_ns = 8.86;
    let managed_diff = (managed_ns_per_op as f64 - target_ns).abs();
    let community_diff = (community_ns_per_op as f64 - target_ns).abs();
    
    println!("\n🎯 Performance Analysis");
    println!("====================");
    println!("🎯 Target: {:.2} ns", target_ns);
    println!("📏 Managed deviation: {:.2} ns", managed_diff);
    println!("📏 Community deviation: {:.2} ns", community_diff);
    
    // Verify Bit 31 logic
    let managed_bit_set = (managed_atom_id & 0x80000000u32) != 0;
    let community_bit_set = (community_atom_id & 0x80000000u32) != 0;
    
    let managed_trust = omwei_atom::get_trust_level(managed_atom_id);
    let community_trust = omwei_atom::get_trust_level(community_atom_id);
    
    println!("\n🔍 Trust Hierarchy Verification");
    println!("==============================");
    println!("🔹 Managed (0x{:08X}): Bit 31 = {} → {:?}", 
             managed_atom_id, managed_bit_set, managed_trust);
    println!("🔹 Community (0x{:08X}): Bit 31 = {} → {:?}", 
             community_atom_id, community_bit_set, community_trust);
    
    // Performance validation
    println!("\n✅ Validation");
    println!("============");
    
    if managed_diff < 10.0 && community_diff < 10.0 {
        println!("🎉 EXCELLENT: Within 10ns of target!");
    } else if managed_diff < 50.0 && community_diff < 50.0 {
        println!("✅ GOOD: Within 50ns of target!");
    } else {
        println!("⚠️  NEEDS OPTIMIZATION: More than 50ns deviation");
    }
    
    // Trust logic validation
    if !managed_bit_set && managed_trust == omwei_atom::TrustLevel::Managed {
        println!("✅ Managed Space logic: CORRECT");
    } else {
        println!("❌ Managed Space logic: INCORRECT");
    }
    
    if community_bit_set && community_trust == omwei_atom::TrustLevel::Community {
        println!("✅ Community Space logic: CORRECT");
    } else {
        println!("❌ Community Space logic: INCORRECT");
    }
    
    println!("\n🏁 Benchmark completed!");
}
