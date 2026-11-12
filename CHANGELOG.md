# Changelog

## 2026-11-12

### Restaurant
- **READY, GO!** splash when you open the restaurant: slide-in art, light rays, a GO! pop with screen shake and sparks, plus a timpani, snare-roll and brass fanfare.
- **Shift Done!** splash when the night ends: the art drops in and bounces, with sweat drops, a collapse thud, plates clinking and a sad-trombone "wah wah wah waaah". Press `E` to skip.
- New driving serving music (brass lead, bass, arpeggio, kick, snare and hats) that speeds up from 128 to 144 BPM at rush hour.
- A night now lasts about 2 minutes instead of 4, with customers arriving 1.5x faster so it stays busy.

### Home
- After the restaurant closes, Mama is back home. She greets you, says you worked hard and tells you to go to sleep.

### Saving
- The current day survives a reload.

## Earlier
- Wave-based NPC customers (groups, queueing, patience, tips, rush hour), the Market Board with a DEX-style swap, pools, stalls, agents, economy and ledger, fishing, hunting, quests and recipes, and the NEAR testnet item ledger.

## 2026-11-12 — docs honesty pass

- README now matches the shipped build: trades are simulated in the browser, the NEP-245 ledger
  contract is deployed, settlement is not enabled (relayer returns 401 without a session).
- Added the measured starting basket, the pool-repricing numbers and a "Try it in two minutes" block.
- Added an MIT LICENSE.

## 2026-11-12 — chain claims re-checked against testnet

- The ledger is not empty: `totals()` reports 18 goods with supply, the three agent accounts hold
  token balances, and `recent(30)` shows transfers up to 2026-11-11 22:02 UTC, 23 of them to
  `*.players.cozy.testnet` player wallets. README now states that instead of implying nothing settled.
- Settlement from this build still fails (`401 no session`); the "403 not your wallet" rule is labelled
  as relayer design, not as a verified claim.
- Dropped "market escrow" — it appears in the relayer's list but is not a deployed testnet account.

## 2026-11-12 — menu music without starting the game

- The menu music now tries to start on page load instead of waiting for a click.
- When the browser blocks autoplay, a "♫ Tap untuk musik" chip appears over the menu: one tap starts
  the music and stays on the menu. Tapping anything that is not a menu entry unlocks it too.
- The "♫ MUSIC ON / OFF" pill is now clickable at z-index above the menu art and updates its label.
