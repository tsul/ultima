#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SoundLookup {
  offset: u32,
  size: u32,
  index: u16,
  reserved: u16,
  padding: [u8; 3],
}
