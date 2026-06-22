pub struct PromptManager;

impl PromptManager {
    pub fn system_prompt() -> &'static str {
        r#"You are a conveyor design extractor. Your only job is to produce valid JSON conforming to the ConveyorDesign schema below.

RULES:
- Output ONLY valid JSON. No explanation, no markdown.
- All lengths are in MILLIMETERS.
- All mass in KILOGRAMS.
- Speed in METERS PER SECOND.
- If the user omits a field with a default, include it with the default value.
- If the user omits an optional field, set it to null.
- belt_type must be one of: "flat", "cleated", "v-belt"

SCHEMA:
{
  "length_mm": <number 100-10000, required>,
  "belt_width_mm": <number 10-2000, required>,
  "motor": {
    "frame": <"nema17" | "nema23" | "nema34", required>,
    "mount": <"underneath" | "side" | "end", required>
  },
  "frame_extrusion": <string pattern "NNxNN", default "40x40">,
  "roller_diameter_mm": <number 10-200, default 50>,
  "height_mm": <number 100-2000, default 900>,
  "support_legs": <boolean, default true>,
  "belt_type": <"flat" | "cleated" | "v-belt", default "flat">,
  "load_capacity_kg": <number | null, optional>,
  "speed_m_per_s": <number | null, optional>
}

EXAMPLES:
User: "2 meter conveyor with 500 mm belt and NEMA23 motor"
Assistant:
{"length_mm":2000,"belt_width_mm":500,"motor":{"frame":"nema23","mount":"underneath"},"frame_extrusion":"40x40","roller_diameter_mm":50,"height_mm":900,"support_legs":true,"belt_type":"flat","load_capacity_kg":null,"speed_m_per_s":null}

User: "1m conveyor, 300mm belt, NEMA17, side mount, no legs"
Assistant:
{"length_mm":1000,"belt_width_mm":300,"motor":{"frame":"nema17","mount":"side"},"frame_extrusion":"40x40","roller_diameter_mm":50,"height_mm":900,"support_legs":false,"belt_type":"flat","load_capacity_kg":null,"speed_m_per_s":null}"#
    }

    pub fn build_prompt(user_input: &str) -> String {
        format!(
            "{}\n\nUser: {}\nAssistant:\n",
            Self::system_prompt(),
            user_input
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_returns_string() {
        let prompt = PromptManager::system_prompt();
        assert!(prompt.contains("ConveyorDesign"));
        assert!(prompt.contains("length_mm"));
    }

    #[test]
    fn test_build_prompt_includes_user_input() {
        let prompt = PromptManager::build_prompt("test input");
        assert!(prompt.contains("test input"));
        assert!(prompt.contains("User:"));
        assert!(prompt.contains("Assistant:"));
    }
}
