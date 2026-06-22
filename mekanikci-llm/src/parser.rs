use mekanikci_core::design::ConveyorSpec;

pub fn parse_conveyor_spec(text: &str) -> anyhow::Result<ConveyorSpec> {
    let text = text.trim();

    if text.is_empty() {
        anyhow::bail!("LLM returned empty output, expected a ConveyorSpec JSON object");
    }

    // Remove markdown code fences if present
    let text = text
        .strip_prefix("```json")
        .or_else(|| text.strip_prefix("```"))
        .and_then(|s| s.strip_suffix("```"))
        .map(|s| s.trim())
        .unwrap_or(text);

    // Find the outermost JSON object
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            let candidate = &text[start..=end];
            if let Ok(spec) = serde_json::from_str::<ConveyorSpec>(candidate) {
                return Ok(spec);
            }
        }
    }

    // Final fallback: try parsing the whole string
    serde_json::from_str::<ConveyorSpec>(text)
        .map_err(|e| anyhow::anyhow!("Failed to parse LLM output as ConveyorSpec: {e}"))
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

    #[test]
    fn test_parse_empty_returns_err() {
        let result = parse_conveyor_spec("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_parse_with_markdown_fences() {
        let json = "```json\n{\"length_mm\": 1000.0, \"belt_width_mm\": 200.0, \"roller_diameter_mm\": 50.0, \"motor\": {\"frame\": \"nema17\", \"mount\": \"underneath\"}}\n```";
        let spec = parse_conveyor_spec(json).unwrap();
        assert_eq!(spec.length_mm, 1000.0);
    }

    #[test]
    fn test_parse_with_leading_text() {
        let json = "Here is the JSON: {\"length_mm\": 1500.0, \"belt_width_mm\": 300.0, \"roller_diameter_mm\": 50.0, \"motor\": {\"frame\": \"nema23\", \"mount\": \"side\"}}";
        let spec = parse_conveyor_spec(json).unwrap();
        assert_eq!(spec.length_mm, 1500.0);
    }
}
