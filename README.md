<p align="center">
  <img src="docs/banner.png" alt="Cozy Hood Trails: fish and hunt by day, run a packed restaurant by night" width="100%">
</p>

<h1 align="center">Cozy Hood Trails</h1>

<p align="center">
  <b>Fish and hunt by day. Cook and serve a packed restaurant by night.</b><br>
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

Play as a fisher and hunter by day, then run a restaurant by night. The game's 22-goods market
changes prices with every trade, and the restaurant's earnings fund your market purchases the next day.

**Play:** https://valenciayuna.github.io/cozy-hood-trails/

## What this is, and the problem it solves

Most cozy sims have a fake market: prices stay fixed and the shop's money comes from nowhere.
This one gives all 22 goods a small real market: each good has its own coin pool, prices shift with every
trade, and the shopkeepers are agents with their own wallets who restock, undercut each other, and
arbitrage. You fish and hunt by day, then run a restaurant by night. The restaurant's earnings fund
your market purchases the next day.

## NEAR HACKATHON submission

- **Track:** `games` — Games With Real Economies
- **Repository:** https://github.com/valenciayuna/cozy-hood-trails (public, MIT)
- **Demo:** posted on X with the demo video, tagged @NEARProtocol and #NEARHackathon
- **Sibling entry:** [Q-DAY](https://github.com/valenciayuna/q-day), an independent project in its own
  repo with no shared code; its README names this one in return (Q-015)

What NEAR does here, concretely:

- the goods ledger is a NEP-245 multi-token contract on testnet
  (`rumah-resto-market-5959.testnet`), deployed and funded, exposing `mt_transfer`,
  `mt_batch_transfer`, `mt_balance_of`, `mt_batch_balance_of`, `mt_tokens_for_owner`, `mt_supply`,
  `recent(n)` and a backend-signed `settle(from, to, token_id, amount)`;
- Market agents are real testnet accounts holding their own goods: `sari.testnet` (produce and
  staples), `bayu.testnet` (fish), and `joko.testnet` (meat and staples). Their holdings are
  verifiable with `mt_tokens_for_owner`, so the ledger records transfers between accounts rather
  than activity in a database;
- trades are recorded as intents (`intent_swap` with `token_in`, `token_out`, `amount_in`, `solver`), a
  shape a solver can route later;
- sign-in is a one-click guest identity: no wallet extension, no seed phrase, no key in the browser.

The contract is deployed and funded. Settlement is **not enabled in this build** (see below), so the
market you play against runs in the browser. This track is judged on the game loop, not the token.

### Baseline and what was built during the window (Q-001, Q-003, Q-004)

The first commit in this repository dates to 2026-11-09, within the hackathon window (which opened
2026-10-19), so there is no pre-hackathon baseline to compare against: every commit here is hackathon
work. `git log --reverse` starts at `3e35c60`.

### Libraries and disclosure (Q-005)

- No framework, template, or UI kit. `index.html` is hand-written HTML/CSS/JS rendered on a canvas;
  it has no external script tags and fetches nothing at runtime. Sound is generated with the Web Audio
  API (no audio files). Two images are embedded in the HTML as data URIs: the menu illustration and
  an NPC sprite sheet. Everything else on screen is drawn procedurally.
- Contract: `near-sdk` 5.30 and `borsh` 1.5. Nothing else.
- Built with AI coding assistants at the author's direction; the game design, art direction and
  economy tuning are the author's.

### Reproducing it (for screening)

"Run it locally" and "Try it in two minutes" above are the full screening walkthrough: open the file,
click Start, swap the same good four times and watch its pool price change, advance one week, then
read the contract through the testnet RPC. No installation, key, or wallet is needed.

## A day in the hood

| Time | What happens |
|---|---|
| **Morning to evening: fish and hunt** | Fish at the dock or waterfall, sneak into the forest to hunt rabbits, hens, boar, and deer, chat with villagers, take quests from the board, and shop at the Merchant Market. |
| **18:00: Mama gets ready** | Mama heads to the restaurant to prep. Choose tonight's menu, learn recipes, and decorate. |
| **19:00–23:00: serve** | Press **Open Restaurant**. Customers arrive in groups, place orders, and quickly lose patience. Pick up each dish from Mama and deliver it to the right table. The door queue grows as the night goes on; last call is at 22:40, and fast service earns tips. A full night takes about 2 minutes. |
| **23:00: shift ends** | Collect the night's earnings, tips, and star rating, then head home. Mama tucks you in for the night. |

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

## Try it in two minutes

1. Open the game and click **Start**. Your guest identity will look like `guestNNNN.testnet`, and
   your starting basket contains 500 coins, 3 carrots, 2 carp, and 6 eggs.
2. Click **MARKET** → **swap**, select `wortel` (carrots), enter `20`, and press the swap button. Repeat four
   times and watch the quoted cost rise (16.4 → 16.9 → 17.5 → 18.1 coins on a fresh guest save):
   the pool reprices after every trade.
3. Press **+1 WEEK**. The weekly report shows updated prices, triggered events, and each agent's
   earnings.
4. Click **LEDGER** to read the deployed contract from the NEAR testnet RPC
   (`rumah-resto-market-5959.testnet`).
5. Play a day: fish and hunt, then open the restaurant at 19:00 and serve until 23:00. Sleeping
   saves your progress; close and reopen the tab to check that it persists.

## Features

- **Restaurant sim.** 4 menu slots, 10 dishes (2 free staples and 8 recipes to learn), table decor, order timers, tips, a nightly summary, and a star rating. Customers arrive in waves, fill tables, and queue at the door when the restaurant is full.
- **Fishing.** Cast, wait, and reel in your catch. Bait improves your odds of finding rare fish, and the waterfall is the best spot for arowana. Grill your catch at the campfire and serve it at the restaurant.
- **Hunting.** Hide, peek, and shoot rabbits, hens, boar, and deer. Stay exposed too long or miss a shot, and the animals will be spooked.
- **A real little economy.** 22 goods, each with its own coin pool and a 0.3% fee. Prices change with every trade, goods-to-goods swaps route through two pools, and random events (bumper harvest, drought, sea storm, livestock plague, village festival, and trade ship) shake things up.
- **Market agents.** Bu Sari, Pak Bayu, Pak Joko, and others produce goods, shop, list items at stalls, and arbitrage using their own wallets. Put your goods up for sale and see who buys them.
- **Fast mode.** Use `+1 HOUR`, `18:00`, and `+1 WEEK` to skip ahead. The weekly report shows price changes, events, and how each agent did.
- **Day and night.** The world darkens at dusk, Mama follows her own daily routine, and your progress is saved in your browser.
- **Fully synthesized audio.** Music for the menu, town, lake, forest, and restaurant, along with sound effects and villagers' voices, is generated live with the Web Audio API. No audio files are used.

## Controls

The game supports keyboard controls on PC and an on-screen pad on phones. You don't need a mouse,
but the on-screen buttons can be clicked too.

### PC (keyboard)

|| Action | Keys |
||---|---|
|| Walk left / right | `A` / `D`, or `←` / `→` |
|| Walk up / down a depth lane (town, house, restaurant floor) | `W` / `S`, or `↑` / `↓` |
|| Interact: talk, open a door, pick up an item, grab a dish, or serve a plate | `E`, `Space`, or `Enter` |
|| Skip one hour | `T` |
|| Jump to 18:00 (Mama starts prepping) | The `18:00` button |
|| Jump ahead one week (market report) | The `+1 WEEK` button |
|| Open / close the Market Board | `M` |
|| Close a panel, or exit fishing or hunting | `Esc` |
|| Fishing: cast, then reel in when ❗ appears | `E` |
|| Hunting: stay hidden / peek out | Hold `Space` or `P` |
|| Hunting: aim | `W`, `A`, `S`, `D`, or the arrow keys |
|| Hunting: shoot | `E` or `Enter` |
|| Mute all audio | The speaker button in the corner |

Movement is continuous while a key is held: keep `D` pressed to keep walking. During hunts, the same
keys steer your crosshair instead of moving your character.

### Phone

- The on-screen pad on the left: `◀ ▶` to walk, `▲ ▼` to change depth, and to aim while hunting.
- The `E` button: interact, cast, reel in, or shoot.
- Hold `PEEK` while hunting. Select `DONE` / `END HUNT` to exit a minigame.
- The HUD buttons `+1 HOUR`, `18:00`, `+1 WEEK`, `MARKET`, and the speaker button work the same as their keyboard controls.

## In development

This demo build does not yet include the following planned features:

| Planned | Status |
|---|---|
| Foraging mode (gather crops/berries in the forest) | not implemented |
| Mining | not implemented |
| Player lobby / multiplayer gathering | not implemented |
| Real wallet login (today: mock guest identity) | not implemented |
| On-chain settlement via the relayer | not enabled (see the status table above) |

## Run it locally

No build step is required. Open `index.html` in a browser, or serve the folder locally:

```bash
python3 -m http.server 8000
# then open http://localhost:8000
```

Every market trade runs through the local AMM. Settlement is designed to use the relayer, but the
session handshake is not yet implemented. The game therefore stays on its local simulation, so a
non-responsive or rejecting endpoint cannot interrupt play.

## Sign in, get a wallet

Sign in as a guest; that is the only sign-in option in this build. There is no OAuth, and the Google
and X buttons on the menu illustration are placeholders that also sign you in as a guest. The menu
and sign-in panel both say so.

Signing in creates a local player identity, such as `guest3058.testnet`, and a starting basket of
500 coins, 3 carrots, 2 carp, and 6 eggs. Your identity and save are stored in
`localStorage`: reloading keeps the same player, while clearing site data or using an incognito window
creates a new one.

The identity is not a NEAR account with a private key, and it cannot be exported or funded. In this
build, your balances are stored in the browser. The on-chain contract is the ledger those goods are
intended to settle against (see the status below).

## What is on chain

| Item | Verified on-chain state | How to check |
|---|---|---|
| Item ledger | `rumah-resto-market-5959.testnet`, a deployed and funded NEP-245 multi-token contract | `view_account` on NEAR testnet RPC |
| Goods on-chain | 18 token IDs with live supply (coin: 6,700; egg: 100; rice: 80; carrot: 40; and more) | `totals()` |
| Market agents | `sari.testnet`, `bayu.testnet`, and `joko.testnet` exist and hold token balances (Sari: apples, onions, spinach, coins, carp, corn, potatoes, eggs, and tomatoes) | `mt_tokens_for_owner` |
| Ledger history | 30 entries in `recent()`, latest at 2026-11-11 22:11 UTC; 23 are transfers to player accounts such as `ce5704793188.players.cozy.testnet` | `recent(30)` |
| Your in-game identity | `guestNNNN.testnet`, stored in `localStorage`; balances are in the local save | Sign in as a guest in the game |
| Trades in this build | AMM pools in the browser, with a 0.3% fee and prices that change on every trade | Swap the same good four times |
| Settlement in this build | **Not enabled:** the relayer responds with `401 {"ok":false,"error":"no session"}` | POST to the relayer's `/trade` |

Click **LEDGER** in game to read the contract's live state from testnet RPC.

So the ledger is real and it has settled before: those `players.cozy.testnet` wallets in the trail are
from the build that issued them. The build in this repo asks for a guest identity and never requests a
session token, so its trades stay in the browser. The game is built to keep playing when the endpoint
refuses; the in-game log (`intent_swap`, `social_login`) records what would have been sent.

## How settlement is guarded

The browser never holds a key. It is built to post `{from, to, token, amount, session}` to the
relayer, which signs `settle` with a function-call key restricted to `settle` and `mt_transfer` on
this contract. The session step is the one missing in this build (see the status table above).

The relayer is designed to enforce three rules:

- a request must carry a valid session token
- your own wallet has to be one side of the trade
- the other side must be an agent wallet, and agent-to-agent trades are refused

What is verified from outside today: a request with no session comes back
`401 {"ok":false,"error":"no session"}`, and `mt_mint` asserts the owner, so a relayer key restricted to
`settle` and `mt_transfer` cannot mint new supply. The wallet-side rule (`403 not your wallet`) is the
relayer's own logic and is not reachable without a session, so treat it as design, not as a tested
claim.

Where goods come from: Bu Sari stocks produce, Pak Bayu stocks fish, Pak Joko stocks meat and
staples. A purchase sends coins to the market wallet and transfers the goods from whichever agent holds them.

## Honest scope

Everything traded in this build is simulated in the browser: the AMM pools, player balances, and prices
are stored in the local save. The game does not write trades to the ledger while the session handshake
is missing. The on-chain state consists of the deployed NEP-245 contract, 18 goods with supply, three
agent accounts with their own balances, and the transfer history left by an earlier build.

Fish and hunted animals are stored in the browser's save. Market ingredients you buy are stored there too.

There is no individual NEAR account, wallet connection, or withdrawal for players. Supply changes
only through trades, never through minting.

## Contract

`rumah-resto-market-5959.testnet` implements NEP-245 multi-token: `mt_mint` (owner only),
`mt_transfer`, `mt_batch_transfer`, `mt_balance_of`, `mt_batch_balance_of`, `mt_tokens_for_owner`,
`mt_supply`, `recent(n)` and `settle(from, to, token_id, amount)` for backend-signed trades.

## Credits

Built for a hackathon by [valenciayuna](https://github.com/valenciayuna). This is one of two independent entries by the same creator. The other is [Q-DAY](https://github.com/valenciayuna/q-day), a post-quantum beat 'em up. The projects have separate repositories and builds, with no shared code.
