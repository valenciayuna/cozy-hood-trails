# Rumah & Resto

A fishing-and-farming restaurant economy on NEAR testnet. Single-file HTML game.

**Play:** https://valenciayuna.github.io/rumah-resto/

## Economy

| piece | where it lives |
|---|---|
| market agents (wallet-enabled NPCs) | in-game simulation, every agent holds its own balance |
| player stalls | in-game, player sets prices, agents buy and sell against them |
| fast mode | `+1 jam`, `18:00`, `+1 Minggu` — months of economy in minutes |
| item ledger | on chain: `rumah-resto-market-5959.testnet` (NEP-245 multi-token) |

Click **LEDGER** in game to read the live ledger: supply, holder balances and the transfer trail,
straight from testnet RPC.

## Contract

`rumah-resto-market-5959.testnet` — NEP-245 multi-token ledger (`contract-src/`):

- `mt_mint(token_id, amount, receiver_id?)` — owner only
- `mt_transfer(receiver_id, token_id, amount)`
- `mt_batch_transfer(receiver_id, token_ids, amounts)`
- `mt_balance_of`, `mt_batch_balance_of`, `mt_tokens_for_owner`, `mt_supply`
- `recent(n)` — bounded transfer trail the market panel renders
- `settle(from, to, token_id, amount)` — owner-signed settlement, so simulated NPC agents can trade

8/8 unit tests pass (`cargo test`). Wasm 220 KB.

## Settlement

Market events leave the browser through a relayer: the game posts `{from, to, token, amount}` to
`POST /trade` and the relayer signs a `settle` call. The browser never holds a key and the player never
sees a wallet popup.

The relayer key is a NEAR **function-call key** restricted to `settle` and `mt_transfer` on this
contract, with a 0.25 NEAR allowance. It cannot mint tokens or move anything else on the account.

What settles on chain:
- player trades against the market (pooled wallet `pemain.resto-market.testnet`)
- NPC agent trades (Bu Sari, Pak Bayu, Pak Joko)

What stays off chain, on purpose:
- **NPC production (`mt_mint`)** — the relayer key cannot mint; supply changes only through trades
- **player balances** — players are pooled into one ledger wallet, not individual accounts
- prices, demand, inflation — simulated in the browser

**Honest scope:** the economy model runs in the browser. Trades settle on chain, player identity does
not. There is no wallet connect and no per-player account.
