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
    --wasm target/wasm32-unknown-unknown/release/token_contract.wasm \
    --id 1 \
    -- \
    balance \
    --id GA7E6OMGZVJFNKTI26NCAMOJFSNDPFHASWD7RPEZZ2C4DMKC526CFDIP
```

**********for testing on testnet need to create account and fund the all account for making transactions

```bash
soroban config identity generate --global alice
soroban config identity address alice #return public address of alice
soroban config identity generate --global acc1
soroban config identity address acc1 #return public address of acc1
soroban config identity generate --global acc2
soroban config identity address acc2 #return public address of acc2
soroban config identity generate --global acc3
soroban config identity address acc3 #return public address of acc3
```

4. Contract deployment and interaction command step for token-contract on <b>testnet</b>(Other way fist install then deploy)
```bash 
soroban contract install \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/token_contract.wasm
```
```bash
soroban contract deploy \
  --wasm-hash 2b22e75ad3bba47451fdde18b4ae9af3a5ba8cbfa22411c71d1da330692e2efa \
  --source alice \
  --network testnet \
  > .soroban/token-contract-id
```


***to Check Particular account balance***
```bash
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
  --source alice \
  --network testnet \
    -- \
    balance \
    --id GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY
```

***to initialize the token contract***
```bash
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
    --source alice \
    --network testnet \
    -- \
    initialize \
    --admin GA7E6OMGZVJFNKTI26NCAMOJFSNDPFHASWD7RPEZZ2C4DMKC526CFDIP \
    --decimal 7 \
    --name "KRISHNA" \
    --symbol "KBR"
```
***To Mint to the token for Particular account for acc1***
```bash    
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
    --source alice \
    --network testnet \
    -- \
    mint \
    --to GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
    --amount 1000000000 \
```
 ***to burn to the token for Particular account for acc1***
```bash    
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
    --source acc1 \
    --network testnet \
    -- \
    burn \
    --from GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
    --amount 1000000 
```

*** to approve account for trnasfer token from one account to another account***

```bash
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
    --source acc1 \
    --network testnet \
    -- \
    approve \
    --from GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
    --spender GANC6CGSVZTJLADFOQFV6VO5XMVHJ6SD5RTJ6MNSHY5W4AN3RYEGPCNO \
    --amount 10000 \
    --expiration_ledger 1849557
```

*** to tranfer fund between one account to another account***
```bash
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
    --source acc1 \
    --network testnet \
    -- \
    transfer \
    --from GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
    --to GANC6CGSVZTJLADFOQFV6VO5XMVHJ6SD5RTJ6MNSHY5W4AN3RYEGPCNO \
    --amount 10000 

```
*** to transfer_from fund between one account to another account ****need  to check again
```bash
soroban contract invoke \
    --id $(cat .soroban/token-contract-id) \
    --source acc1 \
    --network testnet \
    -- \
    transfer_from \
    --spender GA4ECCJTJG53WNJQS5RBXI3TSLNRBPDGWAVNGINKEKYZ73LRSYICXFHY \
    --from GANC6CGSVZTJLADFOQFV6VO5XMVHJ6SD5RTJ6MNSHY5W4AN3RYEGPCNO \
    --to  GD6RDVCWQEAG3PCAPYUB2OCZLCD2O47GFC33MWAYRHIDQTPVZPANUK6A \
    --amount 100 

```

To run test
```bash
cargo test
```
