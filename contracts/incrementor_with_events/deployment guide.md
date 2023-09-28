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
  --wasm target/wasm32-unknown-unknown/release/increment_with_events.wasm \
  --id 2 \
  -- \
  increment
```
***note : decrement is not working in soroban CLI but it working on testnet.
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/increment_with_events.wasm \
  --id 3 \
  -- \
  decrement
```

4. Contract deployment and interaction command step for incrementor on <b>testnet</b>(Other way fist install then deploy)
```bash 
soroban contract install \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/increment_with_events.wasm
```
```bash
soroban contract deploy \
  --wasm-hash 9893aaad3a719bf24537f7010b4d0f979cb6842e6c8126ff6322f0d7c1061fd5 \
  --source alice \
  --network testnet \
  > .soroban/increment_with_events-id
```
7. For invoking 
```bash
soroban contract invoke \
  --id $(cat .soroban/increment_with_events-id) \
  --source alice \
  --network testnet \
  -- \
  increment

  soroban contract invoke \
  --id $(cat .soroban/increment_with_events-id) \
  --source alice \
  --network testnet \
  -- \
  decrement
```

To run test
```bash
cargo test
```

