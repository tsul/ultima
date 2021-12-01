use crate::resources::MulLookup;

pub trait LoadFromMul<L>
where
    Self: std::marker::Sized,
    L: MulLookup,
{
    fn load(id: u16, data: Vec<u8>, lookup: L) -> Option<Self>;
}
