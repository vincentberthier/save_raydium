# Principle

On top of SOL, the tests involve bSOL tokens (from SolBlaze) since they were the easiest tokens known by the Pyth Oracle to get on Devnet.

The test will:
1. Deposit SOL tokens on a Kamino Lending pool to get bSOL
2. Add SOL and bSOL to a Raydium Liquidity pool
3. Remove the liquidity from the pool
4. Repay the borrowed bSOL tokens 

# Dependencies

Just to run the program as is, the only dependency is rust nightly (version >= 1.85.0).

# Public keys (assuming using the given files)

## Static

Those are the keys that only depend on the Save / Raydium protocols

* `5Xs3m9xLbGFYY8C62PxuqAZjwmHnQuAzdjq6xtoKmVbF` - The Kamino Lending program (redeployed)
* `8yrQMUyJRnCJ72NWwMiPV9dNGw465Z8bKUvnUC8P5L6F` - The pyth (oracle) product account
* `BdgHsXrH1mXqhdosXavYxZgX6bGqTdj5mh2sxDhF8bJy` - The pyth (oracle) prices account
* `bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1` - The bSOL Mint

## Dynamic

Those are pubkeys that can be changed (using a different admin pubkeys will change all the other ones). Adjust as needed in that case.

* `EfnT3D1SYM54UbRGX5y6YsYWqqzKDrRXycMxb15pS8Xj` - 'Admin' of the whole thing
* `FtyYfaF1w7qZVHjLwB9mb4mhSjiFh1Fc1dWbQyrhN6dT` - The admin’s ATA of bSOL (solblaze staking tokens)
* `CfqFi1pccHicyH3SKD42UbxK9spK3kpYTc3VGjsfXq6v` - The user address

# Init

You do NOT need to do any of that if you’re using the provided accounts since they already have the SOL and bSOL tokens needed for the tests.

Only do so if you want to change the admin or user’s account. Both accounts will need to have some SOLs (use the (Solana Faucet)[https://faucet.solana.com/]) and bSOL (get some by staking SOLs on (SolBlaze platform)[https://stake.solblaze.org/app/devnet]) for that.
