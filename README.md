# SentinelScope

SentinelScope is an offline defensive security toolkit for inspecting packet captures, IOC lists, IDS-style rules, and local firewall policy exports. It is built as a Rust workspace with reusable parsing crates and small command-line tools for security engineers who need deterministic local analysis without network access.

## Use Cases

- Summarize PCAP files and extract IPv4, TCP, UDP, and DNS metadata.
- Normalize IOC files containing IP addresses, CIDR ranges, domains, URLs, and file hashes.
- Match local IOC collections against packet metadata.
- Inspect Snort/Suricata-like IDS signatures and normalize common fields.
- Audit iptables-style and nftables-style policy exports for duplicate or shadowed rules.

## Supported Artifacts

- Classic PCAP captures with Ethernet frames.
- Ethernet, IPv4, TCP, UDP, and DNS packet data.
- Plain text, CSV, and simple JSON-line IOC entries.
- IDS signatures with action, protocol, endpoints, ports, msg, content, sid, rev, and classtype fields.
- iptables-save style rules, nftables add-rule lines, and simple INI/CSV policy settings.

## Architecture

- `sentinel-core`: byte readers, endian helpers, diagnostics, checksums, string handling, timestamps, and IPv4/CIDR utilities.
- `sentinel-packet`: PCAP, Ethernet, IPv4, TCP, UDP, DNS parsing, packet metadata extraction, and summaries.
- `sentinel-ioc`: IOC parsing, normalization, duplicate handling, allow/block collections, and packet metadata matching.
- `sentinel-rules`: IDS rule lexing, parsing, normalization, validation, and metadata match helpers.
- `sentinel-policy`: firewall policy parsing, validation, ordering analysis, duplicate detection, and summaries.
- `sentinel-cli`: command-line tools that call the library crates.

## Build

```bash
cargo build --workspace
```

## Test

```bash
cargo test --workspace
```

## CLI Usage

```bash
cargo run -p sentinel-cli --bin packetscan -- capture.pcap
cargo run -p sentinel-cli --bin iocmatch -- indicators.txt capture.pcap
cargo run -p sentinel-cli --bin rulecheck -- local.rules
cargo run -p sentinel-cli --bin policyaudit -- firewall.rules
```

All tools read local files only and write deterministic text reports to standard output and standard error.

## Developer Robustness Checks

The `fuzz/` directory contains cargo-fuzz compatible entry points that exercise packet, IOC, IDS rule, and policy parsing paths. These harnesses are developer QA infrastructure for robustness testing.

```bash
cargo fuzz build packet_fuzzer
cargo fuzz run packet_fuzzer fuzz/corpus/packet_fuzzer
cargo fuzz run ioc_fuzzer fuzz/corpus/ioc_fuzzer
cargo fuzz run rules_fuzzer fuzz/corpus/rules_fuzzer
cargo fuzz run policy_fuzzer fuzz/corpus/policy_fuzzer
```

The seed corpus includes small valid PCAPs, malformed near-valid captures, representative IOC lists, IDS rules, and firewall policy snippets. `fuzz/dictionary.txt` contains protocol, IOC, IDS, firewall, JSON, CSV, and INI tokens useful for structured mutation.

## Manual Review Checklist

- Confirm parsers return diagnostics for malformed inputs instead of panicking.
- Review packet length and offset arithmetic around frame, IPv4, TCP, UDP, and DNS parsing.
- Confirm IOC normalization preserves analyst intent and does not over-match allowlists.
- Review IDS rule normalization for stable output and duplicate sid detection.
- Review firewall ordering findings for broad terminal rules and duplicates.
- Run formatting, tests, and robustness checks after parser changes.
