# Chickadee

## Headline

LSM-tree storage engine from scratch. WAL, memtable, SSTables, bloom filters, k-way compaction, and a TCP server on top.

## Category

Learning Project: Database Internals

## What It Is

A key-value database built phase by phase. It started from the [Bitcask paper](https://riak.com/assets/bitcask-intro.pdf) as an append-only log with an in-memory index, then grew into a full LSM tree: a WAL for durability, a `BTreeMap` memtable, SSTable flush, per-SSTable bloom filters, and k-way merge compaction. The architecture behind LevelDB, RocksDB, and Cassandra. A TCP server lets several clients connect at once, and a CLI drives the same engine locally. About 2,200 lines of Rust with 99 tests, from unit tests up to end-to-end TCP.

## What It Proves

- Reading a systems paper and building it, then outgrowing it: Bitcask's append-only log plus an index is the first phase; the LSM tree is where the design ended up once sorted, flushable storage mattered.
- Custom binary format: `[magic: 2B (0x4443 "CD")][crc32: 4B][entry_len: 4B][wincode-serialized Entry]`
- Corruption recovery: byte-by-byte scanning to the next valid header after a bad entry
- Durability: every write goes to the WAL with `sync_all()` before it is acknowledged
- The full LSM read and write paths, tombstones that survive flushes, and compaction that drops them
- Concurrency without overreach: one `Arc<Mutex<Log>>`, locked per command, not per connection

## Key Technical Highlights

### Write Path
Each write appends to the WAL and updates the memtable. Past 4MB the memtable flushes to a new SSTable, named by a zero-padded microsecond timestamp so filename order is age order.

### On-Disk Format
```
[magic: 0x4443] [crc32: 4 bytes] [entry_len: 4 bytes] [wincode Entry]
     "CD"        integrity check    length prefix        serialized data
```

### Read Path
Memtable first, then SSTables newest to oldest. Each SSTable carries a bloom filter (xxh3 double hashing, k=7, 10 bits per key, about 1% false positives), so most misses never touch the file.

### Corruption Recovery
When the magic bytes don't match or the CRC fails, the reader scans forward byte by byte to the next valid header. `CorruptionType` distinguishes NotEnoughBytes, MagicBytesMismatch, ChecksumMismatch, and ParseError.

### Deletes
A delete writes a tombstone to the WAL and memtable. Tombstones are carried through flushes so an older SSTable can't resurrect a deleted key, and compaction drops them once nothing older remains.

### Compaction
Every 10 flushes, all SSTables are merged in one k-way pass over their sorted keys. The newest SSTable wins on duplicate keys, and the compacted output replaces the originals.

### Server
`cargo run --bin server` listens on a TCP port; each connection gets its own thread over the shared, mutex-guarded log. The same `set`/`get`/`delete` commands work over TCP and in the CLI, because both run through one runner that is generic over `BufRead + Write`.

## What I Learned

- How databases actually lay data out on disk: offsets, seeking, length prefixes, binary encoding
- The Bitcask paper, why append-only plus an in-memory index suits write-heavy workloads, and where it stops scaling (the index has to fit in memory)
- Why LSM trees sort and flush: SSTables, bloom filters, and the read amplification compaction exists to pay down
- CRC32 for integrity: what it catches and what it doesn't
- Tombstone semantics, and the resurrection bug you get when a flush drops them

## Status

Complete: all six phases done, from the paper's log to a server clients connect to, with TCP integration tests. A possible next step is leveled compaction (L0/L1) in place of merging every SSTable at once.

## Repo

~/Developer/chickadee
