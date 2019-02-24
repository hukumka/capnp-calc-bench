extern crate capnp;
#[macro_use] 
extern crate capnp_rpc;
extern crate futures;
extern crate tokio;

pub mod calculator_capnp {
  include!(concat!(env!("OUT_DIR"), "/calculator_capnp.rs"));
}

pub mod client;
