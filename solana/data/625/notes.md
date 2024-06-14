# messages.2024-06-09T125348
__Slots:__ from 270049067 to 270431999 for 53 hours.

__LPs__
* Orca: `SOL-USDC and SOL-USDT`
* Raydium: `SOL-USDC, SOL-USDT`
<br/> 365989 rows total.

## Arb config
```
"min_price_spread_threshold": 0.01,
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_base_qty": 0.1,
"min_quote_margin": 1,
 ```

### All LPs
arb.2024-06-10T132454 - 2560 rows total.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  458 |
| "OrcaWhirlpool(SOL-USDC-2)" |  1 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  11 |
| "RaydiumAmm(SOL-USDT)" |  5 |
| "OrcaWhirlpool(SOL-USDC-2)" |  9 |
| "OrcaWhirlpool(SOL-USDC-4)" |  10 |
| "OrcaWhirlpool(SOL-USDC-1)" |  12 |
| "OrcaWhirlpool(SOL-USDT-2)" |  412 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0.1 | 1|459|27656.5|
| 0.5 | 10|2|177.477|
| 1 | 10|||

### Raydium only
arb.2024-06-10T140758 - 1639 rows total.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  21 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  10 |
| "RaydiumAmm(SOL-USDT)" |  11 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|21|1291.77|
| 0.5 | 10|1|88.5267|
| 1 | 10|||

### Raydium: only the USDC LPs
arb.2024-06-10T215632 - 780 rows total.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  10 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  10 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|10|631.675|
| 0.5 | 1|1|88.5267|
| 1 | 10|||

### Raydium: only the liquid USDC/USDT LPs
arb.2024-06-10T144153 - 0 rows total!


## Arb config
```
"min_price_spread_threshold": 0.001, <-- decreased
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_base_qty": 0.1,
"min_quote_margin": 1,
```

### Raydium: only the liquid USDC/USDT LPs
arb.2024-06-10T151650 - 0 rows total!

### All Orca and the only liquid USDC/USDT LPs of Raydium
arb.2024-06-10T154320 - 0 rows total!

### All LPs
arb.2024-06-10T161330 - 2560 rows total. Identical to the `ALL LPs with min_price_spredad_threashold: 0.01`.


## Arb config
Removed min_price_spread_threshold and min_quite_margin.
```
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_quote_margin": 1,
"obsolete_block_threshold": 0,
```

### Raydium: only the USDC LPs.
arb.2024-06-10T224958 - 780 rows total. Identical to arb.2024-06-10T215632 

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  10 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  10 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|10|631.675|
| 0 | 10|10|631.675
| 0.5 | 1|1|88.5267|
| 1 | 1|||
