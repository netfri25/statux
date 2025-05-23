use super::Component;

pub struct Time {
    format: Box<str>,
}

impl Time {
    pub fn new(format: impl Into<Box<str>>) -> Self {
        let format = format.into();
        Self { format }
    }
}

impl Component for Time {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        chrono::Local::now().format(&self.format).write_to(buf)?;
        Ok(())
    }
}
