# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.2] — 2026-04-20

### Changed

- **Dependencies**: Bump workspace dependencies to latest stable
  versions — `uuid` → `1.23`, `tokio` → `1.52`, `sha2` → `0.11`,
  `async-nats` → `0.47`, and `bincode` → `2.0` (the crates.io `3.0.0`
  release is a `compile_error!` stub, so `2.0` is the current usable
  major).
- **`bincode` migration (feature `bincode`)**: migrated the
  `BincodeEventSerializer` and the bincode-gated sequencer tests from
  the legacy `bincode::serialize` / `bincode::deserialize` API to the
  serde bridge in `bincode 2.x`
  (`bincode::serde::encode_to_vec` / `bincode::serde::decode_from_slice`
  with `bincode::config::standard()`). The public
  `EventSerializer` trait and the `BincodeEventSerializer` type are
  unchanged.
- **`sha2` 0.11 compat**: the finalized `Digest` output type no
  longer implements `LowerHex` directly, so
  `OrderBookSnapshotPackage::compute_checksum` now formats the hash
  bytes explicitly.

### Notes

- **Wire-format change (bincode NATS payloads)**: bincode 1.x and
  bincode 2.x produce different byte layouts. Consumers that decoded
  NATS payloads with an older `BincodeEventSerializer` build must
  upgrade to the new version. The on-disk journal is unaffected — it
  uses `serde_json`, not bincode. `ORDERBOOK_SNAPSHOT_FORMAT_VERSION`
  stays at `1`.
- No public API changes — `0.6.2` is a compatible minor release.

## [0.6.1] — 2026-03-22

### Changed

- **Performance**: Replace `Box<dyn Iterator>` with `either::Either`
  for bid/ask iterators, eliminating unnecessary heap allocation and
  dynamic dispatch in the matching hot path.

### Fixed

- Updated dependency management workflows for GitHub Actions

## [0.6.0] — 2025-02-28

### Added

- **NATS JetStream Publishers** (`nats` feature): trade event and book change
  publishers with retry, batching, and throttling.
- **Zero-Copy Serialization** (`bincode` feature): pluggable `EventSerializer`
  trait with JSON and Bincode implementations.
- **Sequencer Subsystem**: `SequencerCommand`, `SequencerEvent`,
  `SequencerResult` types for LMAX Disruptor-style total ordering.
- **Append-Only Journal** (`journal` feature): `FileJournal` with
  memory-mapped segments, CRC32 checksums, and segment rotation.
- **In-Memory Journal**: `InMemoryJournal` for testing and benchmarking.
- **Deterministic Replay**: `ReplayEngine` for disaster recovery and state
  verification from journal.
- **Order State Machine**: `OrderStatus`, `CancelReason`,
  `OrderStateTracker` for explicit lifecycle tracking
  (Open → PartiallyFilled → Filled / Cancelled / Rejected).
- **Order Lifecycle Query API**: `get_order_history()`,
  `active_order_count()`, `terminal_order_count()`,
  `purge_terminal_states()`.
- **Cross-Book Mass Cancel**: `cancel_all_across_books()`,
  `cancel_by_user_across_books()`, `cancel_by_side_across_books()` on
  `BookManager`.
- **Snapshot Config Preservation**: `restore_from_snapshot_package()`
  preserves fee schedule, STP mode, tick/lot size, and order size limits.
- **Clone for OrderBookError**: manual `Clone` impl to work around
  `PriceLevelError` not deriving `Clone`.

### Changed

- Upgraded to **pricelevel v0.7** with `Id`, `Price`, `Quantity`,
  `TimestampMs` newtypes for stronger type safety.
- Removed all `.unwrap()` and `.expect()` from production code.

## [0.5.0] — 2025-01-15

### Added

- **Order Validation**: tick size, lot size, and min/max order size
  validation with configurable limits.
- **Self-Trade Prevention (STP)**: `CancelTaker`, `CancelMaker`,
  `CancelBoth` modes with per-order `user_id` enforcement.
- **Fee Model**: configurable `FeeSchedule` with maker/taker fees and fee
  fields in `TradeResult`.
- **Mass Cancel Operations**: cancel all, by side, by user, by price
  range — with `MassCancelResult` tracking.

## [0.4.8] — 2024-12-20

### Added

- **PriceLevelCache**: faster best bid/ask lookups.
- **MatchingPool**: reduced matching engine allocations.

### Changed

- Refactored modification and matching logic for better separation of
  concerns.
- Improved thread-safe operations under heavy concurrent load.

## [0.4.0] — 2024-11-01

### Added

- **Lock-Free Architecture**: `SkipMap` + `DashMap` + `SegQueue` for
  contention-free concurrent access.
- **Multiple Order Types**: Standard, Iceberg, PostOnly, FillOrKill,
  ImmediateOrCancel, GoodTillDate, TrailingStop, Pegged, MarketToLimit,
  Reserve.
- **Thread-Safe Price Levels**: independent concurrent modification per
  level.
- **Advanced Order Matching**: price-time priority for both market and
  limit orders with partial fills.
- **Multi-Book Management**: `BookManagerStd` and `BookManagerTokio` for
  managing multiple order books.
- **Enriched Snapshots**: single-pass snapshot with VWAP, spread, mid
  price, imbalance, and depth metrics.
- **Implied Volatility**: Black-Scholes implied vol calculation.
- **Market Metrics**: VWAP, micro price, queue analysis, depth
  statistics, and functional iterators.
