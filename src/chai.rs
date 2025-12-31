use ratatui::Frame;

pub trait ChaiApp: Copy + Clone {
    fn new() -> Self;
    fn update(&mut self);
    fn draw(&mut self, f: &mut Frame);
    fn handle_input(&mut self, data: &[u8]);
}
