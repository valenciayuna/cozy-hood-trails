<p align="center">
  <img src="docs/banner.png" alt="Cozy Hood Trails: fish, hunt and forage by day, run a packed restaurant by night" width="100%">
</p>

<h1 align="center">Cozy Hood Trails</h1>

<p align="center">
  <b>Fish, hunt and forage by day. Cook and serve a packed restaurant by night.</b><br>
  A cozy life sim with a living market, playable right in your browser.
</p>

<p align="center">
  <a href="https://valenciayuna.github.io/cozy-hood-trails/"><img src="https://img.shields.io/badge/%E2%96%B6%20PLAY%20NOW-valenciayuna.github.io-e8506e?style=for-the-badge" alt="Play now"></a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/NEAR-testnet-00c08b?style=flat-square" alt="NEAR testnet">
  <img src="https://img.shields.io/badge/NEP--245-multi--token-6f5de8?style=flat-square" alt="NEP-245">
  <img src="https://img.shields.io/badge/single--file-HTML5-f0a030?style=flat-square" alt="Single-file HTML5">
  <img src="https://img.shields.io/badge/install-none-4cae6a?style=flat-square" alt="No install">
</p>

<p align="center">
  <img src="docs/hero.gif" alt="READY, GO! The restaurant opens for the night" width="720">
</p>

A fishing, hunting and foraging life sim with a restaurant and a market where every trade is a real
transaction on NEAR testnet (play money, no real-world value). One HTML file, no install, runs on
desktop and phone.

**Play:** https://valenciayuna.github.io/cozy-hood-trails/

## A day in the hood

| | |
|---|---|
| **Morning to evening: gather** | Fish at the dock or the waterfall, sneak into the forest to hunt rabbits, hens, boar and deer, chat with the villagers, take quests from the board, and shop at the Merchant Market. |
| **18:00: Mama gets ready** | Mama heads to the restaurant to prep. Pick tonight's menu, learn recipes, and decorate. |
| **19:00 to 23:00: serve** | Hit **Open Restaurant**. Customers arrive in groups, order, and lose patience fast. Grab each dish from Mama and run it to the right table. A rush hour kicks in at 21:00 and gets hectic, with tips for quick service. A full night takes about 2 minutes. |
| **23:00: shift done** | Collect the night's earnings, tips and a star rating, then head home. Mama tucks you in and sends you to bed. |

## Screenshots

<table>
  <tr>
    <td width="50%"><img src="docs/screens/13-rush-hour.jpg" alt="Restaurant at rush hour with seated customers and order bubbles"><br><sub><b>Rush hour:</b> every table has an order, Mama is cooking, you are running plates.</sub></td>
    <td width="50%"><img src="docs/screens/04-fishing-village.jpg" alt="Fishing village with a waterfall"><br><sub><b>Fishing village:</b> quest board, bait shop, campfire and the waterfall spot for rare fish.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/screens/05-fishing.jpg" alt="Fishing minigame, reel in"><br><sub><b>Fishing:</b> wait for the sign, then reel in before it gets away.</sub></td>
    <td><img src="docs/screens/06-hunting.jpg" alt="Hunting from a bush, peeking to spot animals"><br><sub><b>Hunting:</b> stay hidden, peek to spot animals, and take your shot.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/screens/03-town.jpg" alt="Town square with villagers and the merchant stall"><br><sub><b>Town:</b> five villagers and the Merchant Market.</sub></td>
    <td><img src="docs/screens/02-inside-home.jpg" alt="Inside the home with Mama"><br><sub><b>Home:</b> Mama, a storage chest and a warm bed.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/screens/11-ready-go-GO.jpg" alt="READY GO splash"><br><sub><b>READY, GO!</b> The night begins.</sub></td>
    <td><img src="docs/screens/14-shift-done.jpg" alt="Shift Done splash"><br><sub><b>Shift done!</b> Time for a well-earned rest.</sub></td>
  </tr>
</table>

<p align="center">
  <img src="docs/market-combo.jpg" alt="Market Board: swap, pools and economy tabs" width="100%"><br>
  <sub><b>Market Board:</b> a DEX-style swap, liquidity pools for every good, and live economy stats.</sub>
</p>

## Features

- **Restaurant sim.** 4 menu slots, 10 dishes (2 free staples plus 8 learnable recipes), table decor, order timers, tips, a nightly summary and a star rating. Customers come in waves, fill tables and queue at the door when the place is full.
- **Fishing.** Cast, wait, reel in. Bait raises rare-fish odds, and the waterfall spot is the best place for arowana. Grill your catch at the campfire and serve it at the restaurant.
- **Hunting.** Hide, peek and shoot at rabbits, hens, boar and deer. The animals get spooked if you stay exposed too long or miss.
- **A real little economy.** 22 goods, each with its own coin pool and a 0.3% fee. Prices move with every trade, goods-to-goods swaps route through two pools, and random events (bumper harvest, drought, sea storm, livestock plague, village festival, trade ship) shake things up.
- **Market agents.** Bu Sari, Pak Bayu, Pak Joko and friends produce, shop, list stalls and arbitrage on their own wallets. Put your own goods on a stall and see who buys.
- **Fast mode.** `+1 HOUR`, `18:00` and `+1 WEEK` skip ahead, and the weekly report shows price changes, events and how each agent did.
- **Day and night.** The world darkens at dusk, Mama has her own daily routine, and your progress saves in the browser.
- **Fully synthesized audio.** The menu, town, lake, forest and restaurant music, the sound effects and even the villagers' voices are generated live with the Web Audio API. There are no audio files.

## Controls

| Action | Keyboard | Phone |
|---|---|---|
| Move | `A` `D` or arrow keys (`W` `S` to move up and down where there is depth) | on-screen pad |
| Interact, talk, serve | `E`, `Space` or `Enter` | `E` button |
| Skip one hour | `T` | `+1 HOUR` |
| Market Board | `M` | `MARKET` |
| Hunting: peek | hold `Space` or `P` | `PEEK` |
| Hunting: aim and shoot | `W` `A` `S` `D` or arrows to aim, `E` or `Enter` to shoot | on-screen pad and `E` |
| Fishing: cast and reel | `E` | `E` button |
| Leave a minigame | `Esc` | `DONE` / `END HUNT` |

## Run it locally

No build step. Open `index.html` in a browser, or serve the folder:

```bash
python3 -m http.server 8000
# then open http://localhost:8000
```

Market settlements go to the relayer when it is reachable. If it is not, the game keeps running on
its local simulation, so a dead endpoint never blocks play.

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

## Credits

Built for a hackathon by [valenciayuna](https://github.com/valenciayuna). This is one of two independent entries by the same builder. The other is [Q-DAY](https://github.com/valenciayuna/q-day), a post-quantum beat 'em up. Different repos, different builds, no shared code.
