## Steps for contract build and Invoke Functions

1. After make changes on smart contract please build first. build will create a target folder wihch contains a .wasm file, which is required for interaction with network.

```bash
cargo test # without logs
cargo test -- --nocapture # with logs
```
```bash
soroban contract build #without logs
soroban contract build --profile release-with-logs #with logs
```
2. Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```

3. Inkvoke Function for increament contract on soroban-cli
```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/storing_custom_data.wasm \
    --id 1 \
    -- \
    increment \
    --incr 5
```
***note : decrement is not working in soroban CLI but it working on testnet.
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/storing_custom_data.wasm \
  --id 2 \
  -- \
  decrement
```

4. Contract deployment and interaction command step for storing_custom_data on <b>testnet</b>(Other way fist install then deploy)
```bash 
soroban contract install \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/storing_custom_data.wasm
```
```bash
soroban contract deploy \
  --wasm-hash 2c63f3a7b0847ed02ac6fa0050d3bf3a0e745926f10f6db2e7686c9ada1face1 \
  --source alice \
  --network testnet \
  > .soroban/storing_custom_data-id
```
7. For invoking 
```bash
soroban contract invoke \
  --id $(cat .soroban/storing_custom_data-id) \
  --source alice \
  --network testnet \
  -- \
  increment \
  --incr 5

  soroban contract invoke \
  --id $(cat .soroban/storing_custom_data-id) \
  --source alice \
  --network testnet \
  -- \
  decrement
```

To run test
```bash
cargo test
```

