use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum NemaFrame {
    #[default]
    Nema17,
    Nema23,
    Nema34,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum MotorMount {
    #[default]
    Underneath,
    Side,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MotorSpec {
    pub frame: NemaFrame,
    pub mount: MotorMount,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motor_spec_deserialize() {
        let json = r#"{"frame": "nema23", "mount": "underneath"}"#;
        let motor: MotorSpec = serde_json::from_str(json).unwrap();
        assert_eq!(motor.frame, NemaFrame::Nema23);
        assert_eq!(motor.mount, MotorMount::Underneath);
    }

    #[test]
    fn test_motor_spec_all_frames() {
        for (json_val, expected) in &[
            ("nema17", NemaFrame::Nema17),
            ("nema23", NemaFrame::Nema23),
            ("nema34", NemaFrame::Nema34),
        ] {
            let json = format!(r#"{{"frame": "{}", "mount": "end"}}"#, json_val);
            let motor: MotorSpec = serde_json::from_str(&json).unwrap();
            assert_eq!(motor.frame, *expected);
        }
    }

    #[test]
    fn test_motor_spec_all_mounts() {
        for (json_val, expected) in &[
            ("underneath", MotorMount::Underneath),
            ("side", MotorMount::Side),
            ("end", MotorMount::End),
        ] {
            let json = format!(r#"{{"frame": "nema23", "mount": "{}"}}"#, json_val);
            let motor: MotorSpec = serde_json::from_str(&json).unwrap();
            assert_eq!(motor.mount, *expected);
        }
    }

    #[test]
    fn test_invalid_frame_rejected() {
        let json = r#"{"frame": "nema999", "mount": "underneath"}"#;
        let result: Result<MotorSpec, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_mount_rejected() {
        let json = r#"{"frame": "nema23", "mount": "sideways"}"#;
        let result: Result<MotorSpec, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
