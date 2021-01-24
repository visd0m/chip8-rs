pub trait Audio {
    fn beep(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop_beep(&mut self);
}
