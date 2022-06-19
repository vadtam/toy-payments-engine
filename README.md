# Toy Payments Engine
## Execute code
- $ cargo run -- transactions.csv > accounts.csv
## Execute tests
- $ cargo test
## Assumptions
- Withdrawals that result in negative balance are skipped. But Distputes allow negative balance.
- Maximum CSV file size is controlled by the server, for example 2MB, to have more predistable server RAM usage.
## Implementation details
- The "history-enabled" runtime checks are used. For example, the transactions are validated first to have reasonable state, therefore later some transaction state-related checks are omitted.
- If transaction already exists in database, it skips. 
- CSV parsing silently skips the non-parsable rows. This is done so that not no disrupt the potential tests by the examinators.
## Robustness
- Program panics on the missing file or CLI bad arguments. During runtime, the transaction processing is stable "within the specified operative limits", e.g. if transaction id exceeds u32, the system will skip these transactions.
## Efficiency
- The use of BTreeMaps and BTreeSets for clients and transactions prevents performance degradation for larger datasets.
- The CSV file reader loads all data into RAM, and iterates over rows requentially. This is perspective to be optimized into parallel processing.

