# Nighthawk

## Headline

LSM-tree storage engine from scratch. WAL, memtable, SSTables, k-way compaction.

## Category

Learning Project — Database Internals

## What It Is

A log-structured storage engine built phase by phase. Started from the Bitcask paper as a simple append-only log, then evolved into a full LSM-tree: BTreeMap memtable, WAL-backed durability, SSTable flush, and k-way merge compaction. The architecture behind LevelDB, RocksDB, and Cassandra. ~800 lines of engine code with 81 tests covering every layer.

## What It Proves

- Understanding of foundational database concepts: append-only logs, in-memory indexes (key → file offset), tombstone deletes, log compaction
- Custom binary format: [magic: 2B (0x4E48 "NH")][crc32: 4B][entry_len: 4B][wincode-serialized Entry]
- Corruption recovery: byte-by-byte scanning to find next valid entry after corruption
- Crash safety: sync_all() after every write, atomic rename for compaction (POSIX semantics)
- Trait-based design: Header trait on File, Index trait on HashMap, Execute trait on Log

## Key Technical Highlights

### On-Disk Format
```
[magic: 0x4E48] [crc32: 4 bytes] [entry_len: 4 bytes] [wincode Entry]
     "NH"        integrity check    length prefix        serialized data
```

### Corruption Recovery
When reading, if magic bytes don't match or CRC fails, scan forward byte-by-byte until the next valid header. Distinguishes HeaderNotFound, MagicBytesNotFound, ChecksumMismatch, and EntryParseError via CorruptionType enum.

### Compaction
Scan all entries, deduplicate (keep latest per key), write to temp file with sync_all(), atomic rename to overwrite original. File handle reopened after merge.

## What I Learned

- How databases actually store data on disk (offsets, seeking, binary encoding)
- Little-endian byte encoding and why it's the convention for on-disk formats (x86/ARM native)
- The Bitcask paper and why append-only + in-memory index is a valid architecture for write-heavy workloads
- CRC32 for data integrity — what it catches and what it doesn't
- POSIX rename semantics for crash-safe file replacement

## Roadmap

Planned phases: SSTable/LSM-tree (sorted on-disk segments, bloom filters), TCP network layer, concurrency with RwLock. Currently through Phase 3 of 6.

## Status

Phases 1-4 complete. WAL, memtable, SSTable flush and read path, and k-way compaction all working with 81 tests. Next: bloom filters and leveled compaction (L0/L1).

## Repo

~/Work/nighthawk
