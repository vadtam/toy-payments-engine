# Toy Payments Engine
## Execute code
- $ cargo run -- transactions.csv > accounts.csv
## Execute tests
- $ cargo test
## Assumptions
- Deposit and Withdrawal Transactions that result in negative values are skipped. But during Disputes, the client assets can be negative.
## Implementation details
- Some runtime checks and tests are omitted as "dublicational", e.g. in the Transaction Resolve function,
the disputed transactions are unwrapped immediately without Optional None check, as they always exist due to the implementation integrity (as the Transaction Resolve fcall is only allowed after Transaction Dispute fcall that already verifies the disputed transaction existence and the transactions themselves are never deleted). This is necessary and enough for this task.
- Also, due to the input validation using the Transaction.validate() call,
some checks on the callstack are omitted due to the implementation integrity (for example, for Transaction Deposit and Transaction Withdraw, the input validation checks that the transaction amount exists and positive, therefore these checks are omitted later in the fcalls).
- CSV parsing silently skips the non-parsable rows. This is done so that not no disrupt the potential tests by the examinators.
## Robustness
- The code is robust. Program panics on the missing file or CLI bad arguments. During runtime, the transaction processing is stable.
- The use of BTreeMaps and BTreeSets for clients and transactions prevents performance degradation for larger datasets.
- The transaction operations (Deposit, Withdraw etc) are programmed reliably, for example, the same transaction cannot be processed twice. Another example is the prohibition to call Resolve on a transaction that is not under dispute. 
## Efficiency
- The CSV FileReader iterates over the CSV file, therefore its RAM requirements do not depend on the CSV file sizes. The Toy Payments Engine iterates the rows and processes the transactions right away, this is a "CSV althernative" to TCP Steaming.
- To process the CSV data concurrently, the CSV file first needs to be loaded into the memory and then concurrently processed. Another option is to iterate over rows and launch light-weight transaction processes.In the Toy Payments Engine, the clients and transactions are stored in the BTreeMaps and BTreeSets, if the concept of concurrent locks or atomic operations exist on these collections (to be checked), then the parallel computations are implementable. But, for the production, I think there will be a SQL database that allows to handle concurrent transactions. 

