## Steps for contract build and Invoke Functions

1. After make changes on smart contract please build first. build will create a target folder wihch contains a .wasm file, which is required for interaction with network.
```bash
cargo test
soroban contract build
```
2. Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```

3. Inkvoke Function for increament contract on soroban-cli
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/incrementor.wasm \
  --id 2 \
  -- \
  increment
```
***note : decrement is not working in soroban CLI but it working on testnet.
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/incrementor.wasm \
  --id 2 \
  -- \
  decrement
```

4. Contract deployment and interaction command step for incrementor on <b>testnet</b>(Other way fist install then deploy)
```bash 
soroban contract install \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/incrementor.wasm
```
```bash
soroban contract deploy \
  --wasm-hash 37cf403dd5c9d86605fe3adabdf3e26fe27e4a5cce0773f9efea990ee2c8a55d \
  --source alice \
  --network testnet \
  > .soroban/incrementor-id
```
7. For invoking 
```bash
soroban contract invoke \
  --id $(cat .soroban/incrementor-id) \
  --source alice \
  --network testnet \
  -- \
  increment

  soroban contract invoke \
  --id $(cat .soroban/incrementor-id) \
  --source alice \
  --network testnet \
  -- \
  decrement
```

To run test
```bash
cargo test
```

