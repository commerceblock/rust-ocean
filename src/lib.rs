// Rust Ocean Library
// Written in 2018 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Ocean Library
//!
//! Extensions to `rust-bitcoin` to support deserialization and serialization
//! of Ocean transactions and blocks.
//!

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

extern crate bitcoin;
#[cfg(feature = "serde")] extern crate serde;

#[macro_use] mod internal_macros;
mod block;
pub mod confidential;
mod transaction;

// export everything at the top level so it can be used as `ocean::Transaction` etc.
pub use transaction::{OutPoint, PeginData, PegoutData, TxIn, TxOut, TxInWitness, TxOutWitness, Transaction, AssetIssuance};
pub use block::{BlockHeader, Block, Proof};

