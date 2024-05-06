# messages.2024-04-25T194857
* Slots 261398784 to 261792013 -> 393229 - the whole epoch 605 after the first hourly snapshot
* Available LPs:
  * Orca: `SOL-USDC`
  * Raydium: `SOL-USDC`
```
"min_price_spread_threshold": 0.01,
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_quote_margin": 1,    
```

### All available LPs
_arb.2024-05-02T165314_ - 1894 records with profit > $1, mostly picking up leftovers from almost empty buy side PLs.

### Raydium SOL-USDC only
_arb.2024-05-02T182426_ - 183 records mostly with low SOL amount on the buy side LPs.

### Orca SOL-USDC only
_arb.2024-05-02T190448_ - just 2 records with in consecutive slots with < $10 profit each.


# messages.2024-04-30T180256
* Slots 261398784 to 261438497 -> 39_713 -> 5 hours only
* Alaviable LPs:
  * Orca: `SOL-USDC, SOL_USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT, bSOL-USDT` 
  * Raydium: `SOL-USDC, SOL-USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT`
```
"min_price_spread_threshold": 0.01,
"max_price_spread_threshold": 0.20,
--> "slippage": 0.005,
"min_quote_margin": 1,
```

### All available LPs
_arb.2024-05-02T144323_ - 892 records with mostly mSOL LPs on the buy sell side. A lot of arbs between Orca Whirlpools only.

### SOL - USDC/USDT only
```
-> "slippage": 0.005,
```
nothing :-(

```
-> "slippage": 0,
```
_arb.2024-05-02T153541_ - 873 records, all with profit < 2

### Radium SOL - USDC/USDT only
```
"slippage": 0,
```
_arb.2024-05-02T155058_ - 55 records, all with profit < 2

### Radium only - all LPs
```
--> "slippage": 0.005,
```
_arb.2024-05-02T163313_ - 524 records, big arbs with mSOL LPs on the buy site unit the slot 261401032 mostly draining the SOL-USDT LP, then a lot of > $10 arbs after slot 261401287.

### Orca only - all LPs
```
--> "slippage": 0.005,
```
_arb.2024-05-02T200938_ - 4486 records with quite big arbs with mSOL LPs on buy site until the slot 261402255, after this a lot of > $10 profits.

### Raydium only, no mSOL
```
--> "slippage": 0.005,
```
_arb.2024-05-02T202851_ - 485 records with bSOL and stSOL on the buy side. Big arbs until the slot 261400984, after this large number of >$10 profits.

### Orca only, no mSOL
```
--> "slippage": 0.005,
```
_arb.2024-05-02T201917_ - 4297 records with bSOL and stSOL on the buy side. No big arbs, but large number of >$10 profits.

