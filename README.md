# Cozy Hood Trails

A fishing, hunting and foraging life sim with a restaurant and a real market, on NEAR testnet.
Single-file HTML game.

**Play:** https://valenciayuna.github.io/cozy-hood-trails/

## What is on chain

| piece | where it lives |
|---|---|
| market agents (wallet-enabled NPCs) | in-game, Bu Sari, Pak Bayu and Pak Joko each hold their own balance |
| player stalls | in-game, prices set by the player, agents buy and sell |
| fast mode | `+1 HOUR`, `18:00`, `+1 WEEK`, months of economy in minutes |
| item ledger | on chain: `rumah-resto-market-5959.testnet` (NEP-245 multi-token) |

Click **LEDGER** in game to read the live ledger: supply, holder balances and the transfer trail,
straight from testnet RPC.

## Settlement

Market events leave the browser through a relayer: the game posts `{from, to, token, amount}` to
`POST /trade` and the relayer signs a `settle` call. The browser never holds a key and the player
never sees a wallet popup.

The relayer key is a NEAR function-call key restricted to `settle` and `mt_transfer` on this
contract, with a 0.25 NEAR allowance. It cannot mint tokens or move anything else on the account.

## Honest scope

The economy model runs in the browser. Trades settle on chain, player identity does not. There is no
wallet connect and no per-player account. NPC production stays simulated: the relayer key cannot
mint, so supply changes only through trades.

## Contract

`rumah-resto-market-5959.testnet` implements NEP-245 multi-token: `mt_mint` (owner only),
`mt_transfer`, `mt_batch_transfer`, `mt_balance_of`, `mt_batch_balance_of`, `mt_tokens_for_owner`,
`mt_supply`, `recent(n)` and `settle(from, to, token_id, amount)` for agent trades signed by the
backend.
