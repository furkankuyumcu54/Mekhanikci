use mekanikci_core::design::ConveyorSpec;

pub fn parse_conveyor_spec(json: &str) -> anyhow::Result<ConveyorSpec> {
    let spec: ConveyorSpec = serde_json::from_str(json)?;
    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_conveyor_spec() {
        let json = r#"{
            "length_mm": 2000.0,
            "belt_width_mm": 500.0,
            "motor": { "frame": "nema23", "mount": "underneath" },
            "frame_extrusion": "40x40",
            "roller_diameter_mm": 50.0,
            "height_mm": 900.0,
            "support_legs": true,
            "belt_type": "flat",
            "load_capacity_kg": null,
            "speed_m_per_s": null
        }"#;
        let spec = parse_conveyor_spec(json).unwrap();
        assert_eq!(spec.length_mm, 2000.0);
    }

    #[test]
    fn test_parse_invalid_json_returns_err() {
        let result = parse_conveyor_spec("not json");
        assert!(result.is_err());
    }
}
