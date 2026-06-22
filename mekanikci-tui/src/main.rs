use std::path::Path;

use anyhow::Context;
use mekanikci_core::backend::{CadBackend, OpenSCADBackend};
use mekanikci_core::design::DesignSpec;
use mekanikci_llm::client::OllamaClient;
use mekanikci_llm::parser::parse_conveyor_spec;
use mekanikci_llm::prompt::PromptManager;
use mekanikci_llm::validation::validate_conveyor_spec;

fn main() -> anyhow::Result<()> {
    let prompt = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: mekanikci-tui <\"natural language prompt\">");
        eprintln!("Example: mekanikci-tui \"2 meter conveyor with 500mm belt and NEMA23\"");
        std::process::exit(1);
    });

    // 1. Build prompt
    println!("Prompt: {prompt}");
    let full_prompt = PromptManager::build_prompt(&prompt);

    // 2. Call Ollama
    println!("Connecting to Ollama...");
    let client = OllamaClient::new("http://127.0.0.1:11434", "qwen3.5:4b", 0.0);
    let json = client
        .generate(&full_prompt)
        .context("Failed to generate response from LLM")?;
    println!("LLM response:\n{json}\n");

    // 3. Parse JSON → ConveyorSpec
    let spec = parse_conveyor_spec(&json)
        .context("Failed to parse LLM output as conveyor specification")?;
    println!(
        "Parsed: {}mm x {}mm, motor: {:?} {:?}",
        spec.length_mm, spec.belt_width_mm, spec.motor.frame, spec.motor.mount
    );

    // 4. Validate
    if let Err(errors) = validate_conveyor_spec(&spec) {
        for e in &errors {
            eprintln!("  Validation error: {} — {}", e.field, e.message);
        }
        anyhow::bail!("Design validation failed");
    }

    // 5. Spec → CAD → STL
    let cad = spec
        .to_cad_model()
        .context("Failed to generate CAD model from spec")?;
    println!("CAD model: {} ({} children)", cad.name, cad.children.len());

    let backend = OpenSCADBackend;
    let output = backend
        .render(&cad, Path::new("./output"))
        .context("Failed to render CAD model")?;

    if let Some(scad) = &output.scad_path {
        println!("SCAD: {}", scad.display());
    }
    if let Some(stl) = &output.stl_path {
        println!("STL:  {}", stl.display());
    }

    Ok(())
}
