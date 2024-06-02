use bitvec::prelude::*;
use crate::error::UserError;

pub fn get_bits(integer: &u64) -> Vec<u64> {
   let bits = integer.view_bits::<Lsb0>();
   let mut bools = Vec::new();
   if let Some(last_one) = bits.last_one() {
      for bit in bits[0..last_one + 1].iter() {
         if *bit {
            bools.push(1);
         } else {
            bools.push(0);
         }
      }
   }
   return bools;
}

pub fn set_bit(integer: &mut u64, index: usize, value: bool) {
   let bits = integer.view_bits_mut::<Lsb0>();
   bits.set(index, value);
}

pub fn get_bit(integer: &u64, index: usize) -> Result<bool, UserError> {
   let bits = integer.view_bits::<Lsb0>();
   match bits.get(index) {
      Some(bit) => Ok(*bit),
      None => Err(UserError::IndexOutOfRange),
   }
}
