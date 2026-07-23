#![deny(clippy::all)]

use hashtree_rs::{hash as HASH, init};
use napi::{bindgen_prelude::Uint8Array, Error};
use std::sync::Once;

#[macro_use]
extern crate napi_derive;

static INIT: Once = Once::new();

#[napi]
pub fn hash(input: Uint8Array) -> Result<Uint8Array, Error> {
  INIT.call_once(|| {
    init();
  });

  let input_len = input.len();
  if !input_len.is_multiple_of(64) {
    return Err(Error::from_reason("Input must be a multiple of 64 bytes"));
  }
  let output_len = input_len / 2;
  let mut output = vec![0u8; output_len];
  HASH(output.as_mut_slice(), input.as_ref(), output_len / 32);
  Ok(Uint8Array::from(output))
}

#[napi]
pub fn hash_into(input: Uint8Array, mut output: Uint8Array) -> Result<(), Error> {
  INIT.call_once(|| {
    init();
  });

  let input_len = input.len();
  if !input_len.is_multiple_of(64) {
    return Err(Error::from_reason(
      "Input length must be a multiple of 64 bytes",
    ));
  }
  let output_len = output.len();
  if output_len != input_len / 2 {
    return Err(Error::from_reason("Output length must be half input"));
  }

  HASH(unsafe { output.as_mut() }, input.as_ref(), output_len / 32);
  Ok(())
}
