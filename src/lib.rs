mod db;
mod errors;
pub mod nibbles;
pub mod node;
pub mod precompile;
mod tests;
mod trie;

pub use db::{MemoryDB, DB};
pub use errors::{MemDBError, TrieError};
pub use trie::{decode_node, EthTrie, RootWithTrieDiff, Trie};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
