use std::fs;

use crate::memory::Memory;

pub fn load_memories() {
    println!("Loading memory...");

    let yaml = fs::read_to_string("datasets/rca/RCA-0003.yaml")
        .expect("Failed to read YAML file");

    let memory: Memory = serde_yaml::from_str(&yaml)
        .expect("Failed to parse YAML");

    println!("ID: {}", memory.id);
    println!("Title: {}", memory.title);
    println!("Category: {}", memory.category);
}
