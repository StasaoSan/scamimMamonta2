# scamimMamonta2
here is dApps lab2

Использовано:
- Rust
- soroban-sdk
- stellar-cli

---

## Установка

Если Rust не установлен:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Установка stellar-cli:
brew install stellar-cli

Добавление target для Soroban:
rustup target add wasm32v1-none

Сборка контракта
stellar contract build

WASM-файл создаётся по пути:
target/wasm32v1-none/release/scamimMamonta2.wasm

Развертывание контракта
stellar contract deploy \
  --wasm target/wasm32v1-none/release/scamimMamonta2.wasm \
  --source stasao1 \
  --network testnet

После выполнения возвращается Contract ID.

Инициализация токена
stellar contract invoke \
  --id CDMPMBGFKGAFXJ3QKRHAGADIGPUHHEONTJJ6MDAT5F4LLHYQWXVSDIXK \
  --source stasao1 \
  --network testnet \
  -- init \
  --owner GAIK2VIDLADEGIMNR3ULWXLQEUTPMOXNBTKD55FVQJHWTK7GF6XTMJ4U \
  --name "Scamim Mamonta" \
  --symbol "SMM" \
  --decimals 0

Эмиссия токенов (mint)
stellar contract invoke \
  --id CDMPMBGFKGAFXJ3QKRHAGADIGPUHHEONTJJ6MDAT5F4LLHYQWXVSDIXK \
  --source stasao1 \
  --network testnet \
  -- mint \
  --to GDI6X6CHBRUGDL4RCBCDZ3WE6FE4NKXZ7VQM37O6VESGGZWV3FE4ESNR \
  --amount 200

Перевод токенов
stellar contract invoke \
  --id CDMPMBGFKGAFXJ3QKRHAGADIGPUHHEONTJJ6MDAT5F4LLHYQWXVSDIXK \
  --source stasao1 \
  --network testnet \
  -- transfer \
  --from GAIK2VIDLADEGIMNR3ULWXLQEUTPMOXNBTKD55FVQJHWTK7GF6XTMJ4U \
  --to GDI6X6CHBRUGDL4RCBCDZ3WE6FE4NKXZ7VQM37O6VESGGZWV3FE4ESNR \
  --amount 50

Проверка баланса
stellar contract invoke \
  --id CDMPMBGFKGAFXJ3QKRHAGADIGPUHHEONTJJ6MDAT5F4LLHYQWXVSDIXK \
  --source stasao1 \
  --network testnet \
  -- balance \
  --id GAIK2VIDLADEGIMNR3ULWXLQEUTPMOXNBTKD55FVQJHWTK7GF6XTMJ4U

Проверка общего количества токенов
stellar contract invoke \
  --id CDMPMBGFKGAFXJ3QKRHAGADIGPUHHEONTJJ6MDAT5F4LLHYQWXVSDIXK \
  --source stasao1 \
  --network testnet \
  -- total_supply

stasao1
GAIK2VIDLADEGIMNR3ULWXLQEUTPMOXNBTKD55FVQJHWTK7GF6XTMJ4U

stasao2
GDI6X6CHBRUGDL4RCBCDZ3WE6FE4NKXZ7VQM37O6VESGGZWV3FE4ESNR

