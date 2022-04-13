# How to run this collator

First start two validators that will run for the relay chain:

```sh
cargo run --release -- -d alice --chain rococo-local --validator --alice --port 50551
cargo run --release -- -d bob --chain rococo-local --validator --bob --port 50552
```

Next start the collator that will collate for the adder dcchain:

```sh
cargo run --release -p test-dcchain-adder-collator -- --tmp --chain rococo-local --port 50553
```

The last step is to register the dcchain using digital_circulation-js. The dcchain id is
100. The genesis state and the validation code are printed at startup by the collator.

To do this automatically, run `scripts/adder-collator.sh`.
