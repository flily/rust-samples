
pub trait Checksum {
    fn checksum(&self) -> u32;
    fn update(&mut self, data: &[u8]) -> usize;
}
