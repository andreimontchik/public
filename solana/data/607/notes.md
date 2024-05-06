# messages.2024-05-04T130921
* Slots from 262261619 to 262348801 -> 87812 - 12 hours
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
_arb.2024-05-06T150934_  - total 136106 rows. Most arbs are between the SOL-USDC-64 and mSOL-USDC-64 Whirlpools. There is $20 difference in prices between these LPs.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  397 |
| "OrcaWhirlpool(SOL-USDC-128)" |  179 |
| "RaydiumAmm(SOL-USDT)" |  3 |
| "OrcaWhirlpool(SOL-USDC-2)" |  486 |
| "OrcaWhirlpool(SOL-USDC-16)" |  52 |
| "OrcaWhirlpool(SOL-USDC-8)" |  108 |
| "OrcaWhirlpool(SOL-USDC-4)" |  400 |
| "OrcaWhirlpool(SOL-USDC-1)" |  552 |
| "OrcaWhirlpool(SOL-USDT-2)" |  339 |
| "OrcaWhirlpool(SOL-USDT-8)" |  39 |
| "OrcaWhirlpool(SOL-USDC-64)" |  41 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(mSOL-USDT)" |  819 |
| "OrcaWhirlpool(mSOL-USDT-64)" |  924 |
| "RaydiumAmm(mSOL-USDC)" |  140 |
| "OrcaWhirlpool(mSOL-USDC-64)" |  713 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|2596|268508|

# Raydium Only
_arb.2024-05-06T152634_ - 5686 rows total. The price difference between buy and sell LPs is $20.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  842 |
| "RaydiumAmm(SOL-USDT)" |  98 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(mSOL-USDT)" |  456 |
| "RaydiumAmm(mSOL-USDC)" |  484 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|940|3.34099e+06|
| 1 | 10|384|3.33456e+06|
| 1 | 100|384|3.33456e+06|

# All LPs for SOL - (USDC | USDT) pairs
_arb.2024-05-06T165231_ -just 4 records between Orca and Raydium LPs.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1 |
 
|Sell LPs|Arbs|
|--------|----|
| "OrcaWhirlpool(SOL-USDC-1)" |  1 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 0|1|1.21473|

# Raydium Only for SOL - (USDC | USDT) pairs
nothing

# Raydium Only without mSOL
_arb.2024-05-06T171842_ - 4384 rows total. Double digits price difference between SOL and stSOL LPs.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  831 |
| "RaydiumAmm(SOL-USDT)" |  87 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(stSOL-USDT)" |  452 |
| "RaydiumAmm(stSOL-USDC)" |  466 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|918|3.20119e+06|
| 1 | 100|372|3.19515e+06|
