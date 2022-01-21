# parse-timecard
## usage
1. set your hourly pay
```rust
const HOURLY_PAY: i64 = /* secret value */;
```
2. cargo build
3. cat [timecard].csv | ./target/debug/parse-timecard
