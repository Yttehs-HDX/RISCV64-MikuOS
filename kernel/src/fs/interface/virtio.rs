pub trait BlockDevice {
    fn read_blocks(&mut self, buf: &mut [u8]);
    fn write_blocks(&mut self, buf: &[u8]);
    fn get_position(&self) -> usize;
    fn set_position(&mut self, position: usize);
    fn move_cursor(&mut self, amount: usize);
}
