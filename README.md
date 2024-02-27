# range-proof-benches

All range proof implementations and its benchmarks in Rust. 

```
# implementation attributes to https://github.com/roynalnaruto/range_proof with minor updates to dependencies.

cd BFDW/
cargo bench

# implementation attributes to https://github.com/MystenLabs/fastcrypto/blob/02d3d4fb096a5575fde71bbd6e0d337245ac1f53/fastcrypto/src/bulletproofs.rs without modification.

cd bulletproofs/
cargo bench

# implementation h/o to https://github.com/novifinancial/hashwires without modification.

cd hashwire/
cargo bench
```