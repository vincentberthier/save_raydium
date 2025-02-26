# Dependencies

1. `solend-program-cli`: cannot be installed from crates.io, needs to be cloned from (github)[https://github.com/solendprotocol/solana-program-library] and built locally (with a rust `1.76.0` at most)
2: `spl-token-cli`: a simple `cargo install spl-token-cli` works just fine.

# Public keys (assuming using the given files)

* `EfnT3D1SYM54UbRGX5y6YsYWqqzKDrRXycMxb15pS8Xj` - 'Admin' of the whole thing
* `ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx` - The save Devnet account
* `CzHgrJsCNMayNCfxLZiyghyasDw3TkDGhJKDHZDQr8qd` - The wrapped sol ATA for the admin account
* `F22LyLxvWs6knJyMyDXgf33kAGXS4BAXf1KrF42u48UZ` - The lending market
* `2rTodi5bwSeNVVWTQyQnufg1QQvPbdd389dkUNPipw3i` - The authority address
* `8yrQMUyJRnCJ72NWwMiPV9dNGw465Z8bKUvnUC8P5L6F` - The pyth (oracle) product account
* `BdgHsXrH1mXqhdosXavYxZgX6bGqTdj5mh2sxDhF8bJy` - The pyth (oracle) prices account

# Init

1. Wrap some sols as an SPL token: `spl-token wrap --fee-payer save_raydium.json 5.0 -- save_raydium.json`, and note the SPL token account pubkey! (it’s `CzHgrJsCNMayNCfxLZiyghyasDw3TkDGhJKDHZDQr8qd` using the save_raydium key)
2. Create a lending market: `solend-cli --program ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx --fee-payer save_raydium.json create-market --market-owner EfnT3D1SYM54UbRGX5y6YsYWqqzKDrRXycMxb15pS8Xj`

# Todo
```bash
solend-program \
  --program      ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx \
  --fee-payer    owner.json \
  add-reserve \
  --market-owner owner.json \
  --source-owner owner.json \
  --market       F22LyLxvWs6knJyMyDXgf33kAGXS4BAXf1KrF42u48UZ \
  --source       CzHgrJsCNMayNCfxLZiyghyasDw3TkDGhJKDHZDQr8qd \
  --amount       5.0  \
  --pyth-product 8yrQMUyJRnCJ72NWwMiPV9dNGw465Z8bKUvnUC8P5L6F \
  --pyth-price   BdgHsXrH1mXqhdosXavYxZgX6bGqTdj5mh2sxDhF8bJy
```
Will give a reserve ID, which will be important to keep to borrow
