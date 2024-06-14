# messages.2024-06-13T021131
* Slots from 270921898 to 271295999 for 51 hour.
* Total rows: 402664

__LPs__
* Orca: `SOL-USDC and SOL-USDT`
* Raydium: `SOL-USDC, SOL-USDT`

## Arb config
```
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_quote_margin": 1,

```

### All LPs
arb.2024-06-14T130318 - 11401 rows total.

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1185 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  46 |
| "OrcaWhirlpool(SOL-USDC-128)" |  1 |
| "RaydiumAmm(SOL-USDT)" |  8 |
| "OrcaWhirlpool(SOL-USDC-2)" |  245 |
| "OrcaWhirlpool(SOL-USDC-16)" |  14 |
| "OrcaWhirlpool(SOL-USDC-8)" |  48 |
| "OrcaWhirlpool(SOL-USDC-4)" |  100 |
| "OrcaWhirlpool(SOL-USDC-1)" |  44 |
| "OrcaWhirlpool(SOL-USDT-2)" |  679 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1185|67991|
| 0.5 | 1|2|220.063|
| 1 | 10|||

## Raydium only
arb.2024-06-14T133424 - 10898 rows total. The total is similar to the `All LPs`, but the outcome of arbs with SOL > 0.5 is better!

__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  86 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDC)" |  69 |
| "RaydiumAmm(SOL-USDT)" |  17 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|86|5349.25|
| 0.5 | 1|3|576.867|
| 1 | 1|1|408.805|


## 2 most active Raydium LPs only
arb.2024-06-14T140940 - 1 row total.
__LP Stats__
|Buy LPs|Arbs|
|-------|----|
| "RaydiumAmm(SOL-USDC)" |  1 |
 
|Sell LPs|Arbs|
|--------|----|
| "RaydiumAmm(SOL-USDT)" |  1 |
 
__Arb Stats__
| Min SOL | Min USD | Arbs | Quote Token Margin |
|---------|---------|------|--------------------|
| 0 | 1|1|558286|