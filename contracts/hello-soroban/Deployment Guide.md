## Steps for contract build and Invoke Functions

Path  is :  cd soroban-tutorial

1. To run test
```bash
cargo test # without logs
cargo test -- --nocapture # with logs
```

2. After make changes on smart contract please build first. build will create a target folder wihch contains a .wasm file, which is required for interaction with network.
```bash
soroban contract build # without logs
soroban contract build --profile release-with-logs # with logs
```
3. Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```
4. Inkvoke Function for hello-soroban contract
```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/hello_soroban.wasm \
    --id 1 \
    -- \
    hello \
    --to friend
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
echo "CCT562R6YIULJ5GDMJEDBBE2B7AFEOBZ5TEXSH4SCVKZCECI2DY7ZGMW" > .soroban/hello-id
soroban contract invoke \
  --id $(cat .soroban/hello-id) \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC
```