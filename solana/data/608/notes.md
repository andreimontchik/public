# messages.2024-05-07T025113
* Slots 262691607 to 262812982 -> 121_375 - 16.8 hours
* LPs:
  * Orca: `SOL-USDC, SOL-USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT, bSOL-USDT` 
  * Raydium: `SOL-USDC, SOL-USDT, mSOL-USDC, mSOL-USDT, stSOL-USDC, stSOL-USDT`
```
"min_price_spread_threshold": 0.01,
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_quote_margin": 1,
```

### All LPs
_arb.2024-05-08T184313_ - 137202 rows total. Most arbs are between SOL and mSOL LPs. The price diff is > $25!

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  621 |
| "RaydiumAmm(SOL-USDT)" |  3 |
| "OrcaWhirlpool(SOL-USDC-2)" |  262 |
| "OrcaWhirlpool(SOL-USDC-16)" |  33 |
| "OrcaWhirlpool(SOL-USDC-8)" |  115 |
| "OrcaWhirlpool(SOL-USDC-4)" |  434 |
| "OrcaWhirlpool(SOL-USDC-1)" |  224 |
| "OrcaWhirlpool(SOL-USDT-2)" |  40 |
| "OrcaWhirlpool(SOL-USDT-8)" |  17 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(mSOL-USDT)" |  443 |
| "OrcaWhirlpool(mSOL-USDT-64)" |  831 |
| "RaydiumAmm(mSOL-USDC)" |  181 |
| "OrcaWhirlpool(mSOL-USDC-64)" |  264 |
| "RaydiumAmm(stSOL-USDC)" |  30 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1749|219539|
| 1 | 10|569|208890|
| 1 | 100|235|193440|

# Raydium Only
_arb.2024-05-08T191045_ - 5274 rows. Same as before. The price diff is insane!

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  188 |
| "RaydiumAmm(SOL-USDT)" |  12 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(mSOL-USDT)" |  145 |
| "RaydiumAmm(mSOL-USDC)" |  52 |
| "RaydiumAmm(stSOL-USDC)" |  3 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|666|1.50412e+06|
| 1 | 10|200|1.4994e+06|
| 1 | 100|200|1.4994e+06|

# All LPs for SOL-(USDC | USDT) pairs
_arb.2024-05-08T192028_ - 725 rows.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  96 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  11 |
| "RaydiumAmm(SOL-USDT)" |  21 |
| "OrcaWhirlpool(SOL-USDC-2)" |  1 |
| "OrcaWhirlpool(SOL-USDC-4)" |  3 |
| "OrcaWhirlpool(SOL-USDT-2)" |  60 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|96|168.359|
| 1 | 10|3|52.5655|

# Raydium Only for SOL-(USDC | USDT) pairs
_arb.2024-05-08T194614_ - 50 rows. Price diff is around $2 at most.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  32 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  11 |
| "RaydiumAmm(SOL-USDT)" |  21 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|32|99.9022|
| 1 | 10|3|52.5655|
