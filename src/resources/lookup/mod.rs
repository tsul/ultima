mod gump_lookup;
mod standard_mul_lookup;

pub use gump_lookup::GumpLookup;
pub use standard_mul_lookup::StandardMulLookup;

pub trait MulLookup
where
    Self: std::marker::Sized + Copy + Clone,
{
    fn from_bytes(data: &[u8]) -> Option<Self>;

    fn offset(&self) -> u64;
    fn size(&self) -> usize;
}
