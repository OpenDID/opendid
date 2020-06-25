use crate::util::XResult;

pub trait SignatureSigner {
    fn sign(&self, message: &[u8]) -> XResult<Vec<u8>>;
}