## Steps for contract build and Invoke Functions


**************Testnet is not giving Proper Error Message as Local soroban cli is giving*******


Path  is :  cd soroban-tutorial

1. To run test
```bash
cargo test
```

2. After make changes on smart contract please build first. build will create a target folder wihch contains a .wasm file, which is required for interaction with network.
```bash
soroban contract build
```
3. Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```
4. Inkvoke Function for increment_with_errors contract
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/increment_with_errors.wasm \
  --id 2 \
  -- \
  increment
```

Steps fo Contract Deployemnt on Testnet

1. Configure CLI for testnet like RPC url and network Passpharse
```bash
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```
2. create a account for contract development 
```bash
soroban config identity generate --global alice
soroban config identity address alice //return public address of alice
```
3. Fund above test account with some test lumens
 curl "https://friendbot.stellar.org/?addr=$(soroban config identity address alice)"
 or go to stellar laboratry  and fund the account.

4. Deploy increment_with_errors contract
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/increment_with_errors.wasm \
  --source alice \
  --network testnet
  ```## Steps for contract build and Invoke Functions

1. After make changes on smart contract please build first. build will create a target folder wihch contains a .wasm file, which is required for interaction with network.
```bash
soroban contract build
```
2. Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```

3. Inkvoke Function for increament contract on soroban-cli
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/increment_with_errors.wasm \
  --id 2 \
  -- \
  increment
```
***note : decrement is not working in soroban CLI but it working on testnet.
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/increment_with_errors.wasm \
  --id 3 \
  -- \
  decrement
```

4. Contract deployment and interaction command step for incrementor on <b>testnet</b>(Other way fist install then deploy)
```bash 
soroban contract install \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/increment_with_errors.wasm
```
```bash
soroban contract deploy \
  --wasm-hash e8cd66c5b552d166c4b720e9d4f3688ca76aaf90882b56d6e529c27304d0c073 \
  --source alice \
  --network testnet \
  > .soroban/increment_with_errors-id
```
7. For invoking 
```bash
soroban contract invoke \
  --id $(cat .soroban/increment_with_errors-id) \
  --source alice \
  --network testnet \
  -- \
  increment

  soroban contract invoke \
  --id $(cat .soroban/increment_with_errors-id) \
  --source alice \
  --network testnet \
  -- \
  decrement
```

To run test
```bash
cargo test
```


