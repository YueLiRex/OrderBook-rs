//! Sequencer subsystem for total-ordered event processing and journaling.
//!
//! This module provides the core types and traits for the single-threaded
//! Sequencer (LMAX Disruptor pattern) and its append-only event journal.
//!
//! # Types
//!
//! - [`SequencerCommand`] — commands submitted for sequenced execution
//! - [`SequencerEvent`] — sequenced events emitted after execution
//! - [`SequencerResult`] — outcomes of command execution
//! - [`JournalError`] — error type for journal operations
//! - [`Journal`] — trait for append-only event journals
//! - [`JournalEntry`] — a single entry read back from the journal
//! - [`in_memory_journal::InMemoryJournal`] — in-memory journal implementation for testing
//! - [`replay::ReplayEngine`] — deterministic replay engine for event journals
//! - [`replay::ReplayError`] — error type for replay operations
//! - `FileJournal` — memory-mapped file journal implementation (requires `journal` feature)
//!
//! # Feature Gate
//!
//! The `FileJournal` implementation requires the `journal` feature:
//!
//! ```toml
//! [dependencies]
//! orderbook-rs = { version = "0.6", features = ["journal"] }
//! ```
//!
//! The sequencer types and [`Journal`] trait are always available.

pub mod error;
pub mod types;

#[cfg(feature = "journal")]
pub mod file_journal;

pub mod in_memory_journal;
pub mod journal;
pub mod replay;

pub use error::JournalError;
#[cfg(feature = "journal")]
pub use file_journal::FileJournal;
pub use in_memory_journal::InMemoryJournal;
pub use journal::{
    ENTRY_CRC_SIZE, ENTRY_HEADER_SIZE, ENTRY_OVERHEAD, Journal, JournalEntry, JournalReadIter,
};
pub use replay::{ReplayEngine, ReplayError, snapshots_match};
pub use types::{SequencerCommand, SequencerEvent, SequencerResult};
