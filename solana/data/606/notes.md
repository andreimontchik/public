# messages.2024-05-02T131611
* Slots 261835693 to 261961947 -> 126254 - 17 hours 
* LPs:
  * Orca: `SOL-USDC, SOL_USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT, bSOL-USDT` 
  * Raydium: `SOL-USDC, SOL-USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT`
```
"min_price_spread_threshold": 0.01,
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_quote_margin": 1,
```
### All LPs
_arb.2024-05-03T150235_ - total 207245 rows. 

__LP Stats__
| Buy Side | Arbs |
|----------|-----:|
| "RaydiumAmm(SOL-USDC)" |  1007 |
| "OrcaWhirlpool(SOL-USDC-128)" |  1 |
| "RaydiumAmm(SOL-USDT)" |  7 |
| "OrcaWhirlpool(SOL-USDC-2)" |  24 |
| "OrcaWhirlpool(SOL-USDC-8)" |  60 |
| "OrcaWhirlpool(SOL-USDC-4)" |  14 |
| "OrcaWhirlpool(SOL-USDC-1)" |  13 |
| "OrcaWhirlpool(SOL-USDT-2)" |  210 |
| "OrcaWhirlpool(SOL-USDT-8)" |  1 |
| "OrcaWhirlpool(SOL-USDC-64)" |  17 |
 
| Sell Side| Arbs |
|----------|-----:|
| "RaydiumAmm(stSOL-USDT)" |  7 |
| "RaydiumAmm(mSOL-USDT)" |  300 |
| "OrcaWhirlpool(SOL-USDC-8)" |  3 |
| "OrcaWhirlpool(mSOL-USDT-64)" |  264 |
| "RaydiumAmm(mSOL-USDC)" |  351 |
| "OrcaWhirlpool(mSOL-USDC-64)" |  423 |
| "RaydiumAmm(stSOL-USDC)" |  6 |

__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin | 
|--------:|--------:|-----:|-------------------:|
| 0 | 1|1354|41357.8|
| 1 | 10|267|29843.2|
| 10 | 100|4|5594.73|

### Orca only
_arb.2024-05-03T202435_ - total 189390 rows. The arb number is higher than total because all Raydium LPs the same token are combined.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "OrcaWhirlpool(SOL-USDC-128)" |  330 |
| "OrcaWhirlpool(SOL-USDC-2)" |  1970 |
| "OrcaWhirlpool(SOL-USDC-16)" |  275 |
| "OrcaWhirlpool(SOL-USDC-8)" |  671 |
| "OrcaWhirlpool(SOL-USDC-4)" |  1623 |
| "OrcaWhirlpool(SOL-USDC-1)" |  2581 |
| "OrcaWhirlpool(SOL-USDT-2)" |  7111 |
| "OrcaWhirlpool(SOL-USDT-8)" |  580 |
| "OrcaWhirlpool(SOL-USDC-64)" |  408 |
 
|Sell LPs|Arbs|
|--------|----|
| "OrcaWhirlpool(mSOL-USDT-64)" |  6028 |
| "OrcaWhirlpool(bSOL-USDT-64)" |  3 |
| "OrcaWhirlpool(mSOL-USDC-64)" |  9518 |

__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|15549|765322|
| 1 | 10|7034|668880|
| 10 | 100|327|105212|

### Raydium only
_arb.2024-05-03T204830_ - total 9741 rows. Raydium LPs are combined by a pair. The number of large amount arbs is higher because the arb algo is prioritizing by the price diff between buy and sell LPs.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1115 |
| "RaydiumAmm(SOL-USDT)" |  14 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(stSOL-USDT)" |  7 |
| "RaydiumAmm(SOL-USDC)" |  1 |
| "RaydiumAmm(mSOL-USDT)" |  425 |
| "RaydiumAmm(mSOL-USDC)" |  690 |
| "RaydiumAmm(stSOL-USDC)" |  6 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1129|418681|
| 1 | 10|57|407119|
| 10 | 100|32|404046|

### Orca and Raydium, but for SOL - (USDC | USDT) pairs only
_arb.2024-05-03T211739_ - total 14706 rows.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1157 |
| "RaydiumAmm(SOL-USDT)" |  6 |
| "OrcaWhirlpool(SOL-USDC-1)" |  1 |
| "OrcaWhirlpool(SOL-USDT-8)" |  2 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  22 |
| "OrcaWhirlpool(SOL-USDC-128)" |  6 |
| "RaydiumAmm(SOL-USDT)" |  56 |
| "OrcaWhirlpool(SOL-USDC-2)" |  347 |
| "OrcaWhirlpool(SOL-USDC-16)" |  7 |
| "OrcaWhirlpool(SOL-USDC-8)" |  103 |
| "OrcaWhirlpool(SOL-USDC-4)" |  330 |
| "OrcaWhirlpool(SOL-USDC-1)" |  102 |
| "OrcaWhirlpool(SOL-USDT-2)" |  177 |
| "OrcaWhirlpool(SOL-USDC-64)" |  16 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1166|1476.59|
| 1 | 10|2|34.6987|
| 10 | 100| n/a|n/a|

### Orca only for SOL - (USDC | USDT) pairs
_arb.2024-05-03T221932_ - total 5 rows.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "OrcaWhirlpool(SOL-USDC-64)" |  3 |
 
|Sell LPs|Arbs|
|--------|----|
| "OrcaWhirlpool(SOL-USDC-4)" |  3 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|3|10.4293|


### Raydium only for SOL - (USDC | USDT) pairs
_arb.2024-05-03T223255_ - total 397 rows.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  119 |
| "RaydiumAmm(SOL-USDT)" |  8 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  43 |
| "RaydiumAmm(SOL-USDT)" |  84 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|127|2778.96|
| 10 | 100|1|2621.15|



