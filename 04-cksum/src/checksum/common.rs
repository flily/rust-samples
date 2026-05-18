pub trait Checksum {
    fn build() -> Box<dyn Checksum> where Self: Sized;
    fn checksum(&self) -> u32;
    fn update(&mut self, data: &[u8]) -> usize;
}
