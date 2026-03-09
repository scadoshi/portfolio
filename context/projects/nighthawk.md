# Nighthawk

## Headline

Log-structured key-value store built from scratch — append-only log, binary format, CRC32 checksums, corruption recovery.

## Category

Learning Project — Database Internals

## What It Is

A key-value store modeled after the Bitcask paper. Append-only log with in-memory HashMap index, binary serialization with custom headers (magic bytes, CRC32, length-prefixed entries), log compaction via atomic rename, and byte-by-byte corruption recovery.

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

Learning project. Phases 1-3 complete (append-only log, compaction, binary serialization with checksums).

## Repo

~/Work/nighthawk
