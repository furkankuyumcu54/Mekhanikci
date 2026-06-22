mod openscad;

pub use openscad::OpenSCADBackend;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cad::CadAssembly;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutputFiles {
    pub scad_path: Option<PathBuf>,
    pub stl_path: Option<PathBuf>,
    pub step_path: Option<PathBuf>,
    pub bom_path: Option<PathBuf>,
}

pub trait CadBackend {
    fn render(&self, model: &CadAssembly, output_dir: &Path) -> anyhow::Result<OutputFiles>;
}
