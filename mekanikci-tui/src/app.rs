pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        anyhow::bail!("App::run not implemented yet")
    }
}
