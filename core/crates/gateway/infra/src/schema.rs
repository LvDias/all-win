//! Adapter-protocol frame validation against the canonical schema (spec-first, ADR-0002 §4).
//! The validator is compiled once and is immutable afterwards, so concurrent sessions share it
//! safely (the plugin's flake T1 came from a mutable shared validator).

use gateway_application::frames::FrameValidator;
use serde_json::Value;

const SCHEMA_JSON: &str =
    include_str!("../../../../../contracts/adapter-protocol/v1/messages.schema.json");
const INSTRUCTION_SCHEMA_JSON: &str =
    include_str!("../../../../../contracts/instruction/v1/instruction.schema.json");
const INSTRUCTION_SCHEMA_ID: &str =
    "https://allwin.app/contracts/instruction/v1/instruction.schema.json";

pub struct ProtocolSchema {
    validator: jsonschema::Validator,
}

impl ProtocolSchema {
    /// Fails only if the embedded schemas are broken (a build defect).
    pub fn load() -> Result<Self, String> {
        let schema: Value = serde_json::from_str(SCHEMA_JSON).map_err(|e| e.to_string())?;
        let instruction: Value =
            serde_json::from_str(INSTRUCTION_SCHEMA_JSON).map_err(|e| e.to_string())?;
        let registry = jsonschema::Registry::new()
            .add(INSTRUCTION_SCHEMA_ID, instruction)
            .map_err(|e| e.to_string())?
            .prepare()
            .map_err(|e| e.to_string())?;
        let validator = jsonschema::options()
            .with_registry(&registry)
            .build(&schema)
            .map_err(|e| e.to_string())?;
        Ok(Self { validator })
    }
}

impl FrameValidator for ProtocolSchema {
    /// The error names only the instance path, never a value from the frame.
    fn validate(&self, frame: &Value) -> Result<(), String> {
        match self.validator.iter_errors(frame).next() {
            None => Ok(()),
            Some(e) => Err(format!("schema violation at {}", e.instance_path())),
        }
    }
}
