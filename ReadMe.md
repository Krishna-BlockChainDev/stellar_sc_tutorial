## Steps for contract build and Invoke Functions

After make changes on smart contract please build first.
```bash
soroban contract build
```
Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```
Inkvoke Function for token contract
```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_contract.wasm \
    --id 1 \
    -- \
    hello \
    --to friend
```

Inkvoke Function for increament contract
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/incrementor.wasm \
  --id 2 \
  -- \
  increment
```

```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/incrementor.wasm \
  --id 2 \
  -- \
  decrement
```
To run test
```bash
cargo test
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

4. Deploy hello-soroban contract
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_soroban.wasm \
  --source alice \
  --network testnet
  ```
5. Interact With contract first save the deployed address in a file 
```bash
echo "C…paste deployed address" > .soroban/hello-id
soroban contract invoke \
  --id $(cat .soroban/hello-id) \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC
```

6. Contract deployment and interaction command step for incrementor(Other way fist install then deploy)
```bash 
soroban contract install \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/incrementor.wasm
```
```bash
soroban contract deploy \
  --wasm-hash [paste the output from the last command] \
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


