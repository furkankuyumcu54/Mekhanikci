use mekanikci_core::design::ConveyorSpec;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub fn validate_conveyor_spec(spec: &ConveyorSpec) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if spec.length_mm < 100.0 || spec.length_mm > 10000.0 {
        errors.push(ValidationError {
            field: "length_mm".into(),
            message: format!("must be 100–10000, got {}", spec.length_mm),
        });
    }

    if spec.belt_width_mm < 10.0 || spec.belt_width_mm > 2000.0 {
        errors.push(ValidationError {
            field: "belt_width_mm".into(),
            message: format!("must be 10–2000, got {}", spec.belt_width_mm),
        });
    }

    if spec.roller_diameter_mm < 10.0 || spec.roller_diameter_mm > 200.0 {
        errors.push(ValidationError {
            field: "roller_diameter_mm".into(),
            message: format!("must be 10–200, got {}", spec.roller_diameter_mm),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mekanikci_core::design::{MotorMount, MotorSpec, NemaFrame};

    fn make_spec(
        length_mm: f64,
        belt_width_mm: f64,
        roller_diameter_mm: f64,
    ) -> ConveyorSpec {
        ConveyorSpec {
            length_mm,
            belt_width_mm,
            motor: MotorSpec {
                frame: NemaFrame::Nema23,
                mount: MotorMount::Underneath,
            },
            frame_extrusion: "40x40".into(),
            roller_diameter_mm,
            height_mm: 900.0,
            support_legs: true,
            belt_type: "flat".into(),
            load_capacity_kg: None,
            speed_m_per_s: None,
        }
    }

    #[test]
    fn test_valid_spec_passes() {
        let spec = make_spec(2000.0, 500.0, 50.0);
        assert!(validate_conveyor_spec(&spec).is_ok());
    }

    #[test]
    fn test_length_too_low_fails() {
        let spec = make_spec(0.0, 500.0, 50.0);
        let err = validate_conveyor_spec(&spec).unwrap_err();
        assert!(err.iter().any(|e| e.field == "length_mm"));
    }

    #[test]
    fn test_belt_width_too_high_fails() {
        let spec = make_spec(2000.0, 3000.0, 50.0);
        let err = validate_conveyor_spec(&spec).unwrap_err();
        assert!(err.iter().any(|e| e.field == "belt_width_mm"));
    }

    #[test]
    fn test_roller_diameter_invalid_fails() {
        let spec = make_spec(2000.0, 500.0, 5.0);
        let err = validate_conveyor_spec(&spec).unwrap_err();
        assert!(err.iter().any(|e| e.field == "roller_diameter_mm"));
    }

    #[test]
    fn test_multiple_errors_collected() {
        let spec = make_spec(0.0, 0.0, 0.0);
        let err = validate_conveyor_spec(&spec).unwrap_err();
        assert!(err.len() >= 2);
    }
}
