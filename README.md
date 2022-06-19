# Toy Payments Engine
## Execute code
- $ cargo run -- transactions.csv > accounts.csv
## Execute tests
- $ cargo test
## Assumptions
- Withdrawals that result in negative balance are skipped.
- During Disputes, client assets can be negative.
- Maximum CSV file size is controlled by the server, for example 2MB, to have more predictable server RAM usage.
- If an incoming transaction already exists in database, it is skipped. 
- CSV parsing silently skips the non-parsable rows. This is done so that not no disrupt the potential tests by the examinators.
## Implementation details
- The "history-enabled" runtime checks are used. For example, the transactions are validated first to have reasonable state, therefore later some transaction state-related checks are omitted as being redundant.
## Robustness
- Program panics on the missing file or CLI bad arguments.
- During runtime, the transaction processing is stable "within the specified operative limits", e.g. if a transaction's id exceeds u32, the system will skip this transaction.
## Efficiency
- The use of BTreeMaps and BTreeSets for clients and transactions prevents performance degradation for larger datasets.
- The CSV file reader loads all data into RAM and iterates over rows sequentially. For the REST async servers with many requests of small transaction lists, this is acceptable. However, if the incoming transaction lists become large, the issue of threading them arises.

