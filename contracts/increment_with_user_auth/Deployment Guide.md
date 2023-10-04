## Steps for contract build and Invoke Functions


Path  is :  cd soroban-tutorial

1. To run test
```bash
cargo test # without logs
cargo test -- --nocapture # with logs
```

2. After make changes on smart contract please build first. build will create a target folder wihch contains a .wasm file, which is required for interaction with network.
```bash
soroban contract build #without logs
soroban contract build --profile release-with-logs #with logs commands not working
```
3. Once Its build successfully, A .wasm file will created successfully you can verify by following command.
```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```
4. Inkvoke Function for increment_with_user_auth contract.

```bash
    soroban contract invoke \
    --source acc1 \
    --wasm target/wasm32-unknown-unknown/release/increment_with_user_auth.wasm \
    --id 1 \
    -- \
    increment \
    --user GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
    --value 2
```
```bash
    soroban contract invoke \
    --source acc2 \
    --wasm target/wasm32-unknown-unknown/release/increment_with_user_auth.wasm \
    --id 1 \
    -- \
    increment \
    --user GANC6CGSVZTJLADFOQFV6VO5XMVHJ6SD5RTJ6MNSHY5W4AN3RYEGPCNO \
    --value 5
```

Steps fo Contract Deployemnt on Testnet

1. Configure CLI for testnet like RPC url and network Passpharse
```bash
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```
2. create two user account for contract development 
```bash
soroban config identity generate acc1 && \
soroban config identity generate acc2 && \
soroban config identity address acc1 && \
soroban config identity address acc2
```
3. Fund above test account with some test lumens
 curl "https://friendbot.stellar.org/?addr=$(soroban config identity address alice)"
 or go to stellar laboratry  and fund the account.


4. Deploy increment_with_user_auth contract
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/increment_with_user_auth.wasm \
  --source alice \
  --network testnet
  ```
5. Interact With contract first save the deployed address in a file 
```bash
echo "CDKJPWCS7JLDRJOL26LJJMXA6YLNRBE6S6K5CZZMAF3TZMZ5KKNBKPLY" > .soroban/increment_with_user_auth-id
```

6. For invoking with acc1 and acc2
```bash
  soroban contract invoke \
  --id $(cat .soroban/increment_with_user_auth-id) \
  --source acc1 \
  --network testnet \
  -- \
  increment \
  --user GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
  --value 2


  soroban contract invoke \
  --id $(cat .soroban/increment_with_user_auth-id) \
  --source acc2 \
  --network testnet \
  -- \
  increment \
   --user GANC6CGSVZTJLADFOQFV6VO5XMVHJ6SD5RTJ6MNSHY5W4AN3RYEGPCNO \
  --value 2

  soroban contract invoke \
  --id $(cat .soroban/increment_with_user_auth-id) \
  --source acc2 \
  --network testnet \
  -- \
  decrement \
   --user GANC6CGSVZTJLADFOQFV6VO5XMVHJ6SD5RTJ6MNSHY5W4AN3RYEGPCNO \
  --value 2
```

To run test
```bash
cargo test
```


