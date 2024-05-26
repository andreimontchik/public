# messages.2024-05-23T150622
Slots 266585575 to 266975998 -> 390423 - 54 hours

* LPs:
  * Orca: `SOL-USDC, SOL-USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT, bSOL-USDT` 
  * Raydium: `SOL-USDC, SOL-USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT`

## Arb config:
    ```
    "min_price_spread_threshold": 0.01,
    "max_price_spread_threshold": 0.20,
    "slippage": 0.005,
    "min_base_margin": 0.01,
    "min_quote_margin": 1,
    ```

### All LPs
arb.2024-05-24T140139 - 4520 rows total. The replayer was not checking for every new slot, that is they there are gaps between slots. The mSOL-USD[C,T] price is ridiculously high!

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1052 |
| "OrcaWhirlpool(SOL-USDC-128)" |  4 |
| "RaydiumAmm(SOL-USDT)" |  5 |
| "OrcaWhirlpool(SOL-USDC-2)" |  6 |
| "OrcaWhirlpool(SOL-USDC-16)" |  2 |
| "OrcaWhirlpool(SOL-USDC-8)" |  3 |
| "OrcaWhirlpool(SOL-USDC-4)" |  1 |
| "OrcaWhirlpool(SOL-USDC-1)" |  2 |
| "OrcaWhirlpool(SOL-USDT-2)" |  12 |
| "OrcaWhirlpool(SOL-USDT-8)" |  8 |
| "OrcaWhirlpool(SOL-USDC-64)" |  7 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(stSOL-USDT)" |  5 |
| "RaydiumAmm(mSOL-USDT)" |  209 |
| "OrcaWhirlpool(mSOL-USDT-64)" |  267 |
| "RaydiumAmm(mSOL-USDC)" |  221 |
| "OrcaWhirlpool(stSOL-USDC-64)" |  4 |
| "OrcaWhirlpool(mSOL-USDC-64)" |  390 |
| "RaydiumAmm(stSOL-USDC)" |  6 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1102|74659.9|

### Raydium Only
arb.2024-05-24T151346 - 247419 rows total. Arbs in more than half of the slots between SOL and (m|st)SOL LPs.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1535 |
| "RaydiumAmm(SOL-USDT)" |  40 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(stSOL-USDT)" |  4 |
| "RaydiumAmm(mSOL-USDT)" |  660 |
| "RaydiumAmm(mSOL-USDC)" |  902 |
| "RaydiumAmm(stSOL-USDC)" |  9 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1575|868262|
| 1 | 10|82|850265|
| 10 | 100|54|846470|

### All LPs for SOL-(USDC | USDT) pairs
arb.2024-05-24T155746 - 13105 rows total.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  2927 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  50 |
| "OrcaWhirlpool(SOL-USDC-128)" |  1 |
| "RaydiumAmm(SOL-USDT)" |  45 |
| "OrcaWhirlpool(SOL-USDC-2)" |  587 |
| "OrcaWhirlpool(SOL-USDC-16)" |  57 |
| "OrcaWhirlpool(SOL-USDC-8)" |  325 |
| "OrcaWhirlpool(SOL-USDC-4)" |  585 |
| "OrcaWhirlpool(SOL-USDC-1)" |  544 |
| "OrcaWhirlpool(SOL-USDT-2)" |  697 |
| "OrcaWhirlpool(SOL-USDC-64)" |  36 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|2927|4111.99|
| 1 | 10|3|108.597|
| 10 | 100|||

### Raydium Only for SOL-(USDC | USDT) pairs
arb.2024-05-24T164934 - 12767 records.

|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  236 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  164 |
| "RaydiumAmm(SOL-USDT)" |  72 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|236|342.878|
| 1 | 10|||

## Arb config:
    ```
    "min_price_spread_threshold": 0.01,
    "max_price_spread_threshold": 0.20,
    "slippage": 0.005,
    "min_base_qty": 1, <-- added to the price filter
    "min_quote_margin": 1,
    ```
### All LPS for SOL-(USDC | USDT) pairs
arb.2024-05-26T133415 - 34 rows total. Mostly between Orca and Raydium LPs. Usually last for few slots.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  7 |
| "OrcaWhirlpool(SOL-USDC-128)" |  2 |
| "OrcaWhirlpool(SOL-USDC-2)" |  2 |
| "OrcaWhirlpool(SOL-USDC-4)" |  3 |
| "OrcaWhirlpool(SOL-USDT-2)" |  7 |
| "OrcaWhirlpool(SOL-USDC-64)" |  1 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  8 |
| "RaydiumAmm(SOL-USDT)" |  6 |
| "OrcaWhirlpool(SOL-USDC-4)" |  1 |
| "OrcaWhirlpool(SOL-USDC-64)" |  7 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|22|245.43|
| 1 | 10|6|173.52|

### Raydium Only for SOL-(USDC | USDT) pairs
arb.2024-05-24T182932 - 27 rows total. Arbs are usually getting scooped up whithin 3-5 slots.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  10 |
| "RaydiumAmm(SOL-USDT)" |  5 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  13 |
| "RaydiumAmm(SOL-USDT)" |  2 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|15|216.018|
| 1 | 10|6|166.738|
| 10 | 100|||

## Arb config:
    ```
    "min_price_spread_threshold": 0.01,
    "max_price_spread_threshold": 0.20,
    "slippage": 0.005,
    "min_base_qty": 0.5, <-- decreased 
    "min_quote_margin": 1,
    ```
### Raydium Only for SOL-(USDC | USDT) pairs
arb.2024-05-24T195648 - 577 rows total. The total amount of arbs is less, but the arb lengh is much longer and with min_base_qty=1.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  26 |
| "RaydiumAmm(SOL-USDT)" |  5 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  24 |
| "RaydiumAmm(SOL-USDT)" |  7 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|31|123.641|
| 1 | 10|2|47.059|
| 10 | 100|||

## Arb config:
    ```
    "min_price_spread_threshold": 0.01,
    "max_price_spread_threshold": 0.20,
    "slippage": 0.005,
    "min_base_qty": 0.2, <-- decreased 
    "min_quote_margin": 1,
    ```

### Raydium Only for SOL-(USDC | USDT) pairs
arb.2024-05-24T204259 - rows total. The total amount and the arbs length bigger thanwith min_base_qty=(1 | 0.5).

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  236 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  164 |
| "RaydiumAmm(SOL-USDT)" |  72 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|236|342.878|
| 1 | 10|||
