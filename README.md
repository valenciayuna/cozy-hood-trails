# Cozy Hood Trails

A fishing, hunting and foraging life sim with a restaurant and a market where the money is real, on
NEAR testnet. Single-file HTML game.

**Play:** https://valenciayuna.github.io/cozy-hood-trails/

## Sign in, get a wallet

Sign in as a guest. That is the only path in this build: there is no OAuth, and the Google and X
plates on the menu picture are placeholders that sign you in as a guest too. The menu says so, and so
does the sign-in panel.

The relayer issues your browser a ledger wallet on the spot, something like
`fbd3a75c23fa.players.cozy.testnet`, and hands over a starting basket: 120 coin, 2 wortel, 2 telur.
The identity lives in `localStorage`, so reloading keeps the same wallet, and clearing site data or
using incognito gives you a new one.

The wallet is not a keyed account you can export. It is a balance held in the contract, addressed by
your browser's identity, and moved only by the relayer on your behalf. Progress saves to the browser
as well: same browser, same save and same wallet.

## What is on chain

| piece | where it lives |
|---|---|
| your wallet | contract state, `mt_balance_of` under your issued account |
| market agents | Bu Sari, Pak Bayu and Pak Joko each hold their own on-chain balance |
| trades | every buy, sell and stall sale settles with a real transaction |
| item ledger | `rumah-resto-market-5959.testnet`, NEP-245 multi-token |
| fast mode | `+1 HOUR`, `18:00`, `+1 WEEK` compress months into minutes |

Click **LEDGER** in game to read live balances and the transfer trail from testnet RPC.

## How settlement is guarded

The browser never holds a key. It posts `{from, to, token, amount, session}` to the relayer, which
signs `settle` with a function-call key restricted to `settle` and `mt_transfer` on this contract.

Three rules stop the endpoint from being a tap into the agents' money:

- a request must carry a valid session token
- your own wallet has to be one side of the trade
- the other side must be an agent wallet, and agent-to-agent trades are refused

So a crafted request like `{from: sari.testnet, to: me}` comes back `403 not your wallet`, and the
relayer key cannot mint, so it cannot inflate supply.

Where goods come from: Bu Sari stocks produce, Pak Bayu stocks fish, Pak Joko stocks meat and
staples. A purchase moves coin to the market wallet and the goods from whichever agent actually
holds them.

## Honest scope

Your on-chain wallet tracks what you traded. It does not track what you gathered: fish you catch and
crops you pick live in the browser's save. Selling gathered goods still settles, because the relayer
credits the goods leg and pays coin, but the chain never saw them arrive.

There is no per-player NEAR account, no wallet connect and no withdrawal. Supply changes only
through trades, never through minting.

## Contract

`rumah-resto-market-5959.testnet` implements NEP-245 multi-token: `mt_mint` (owner only),
`mt_transfer`, `mt_batch_transfer`, `mt_balance_of`, `mt_batch_balance_of`, `mt_tokens_for_owner`,
`mt_supply`, `recent(n)` and `settle(from, to, token_id, amount)` for backend-signed trades.
