# messages.2024-06-20T163456
* Slots from 272193592 to 272592007 for 55 hours.
* Total rows: 421031

__LPs__
* Orca: `SOL-USDC and SOL-USDT`
* Raydium: `SOL-USDC, SOL-USDT`

## Arb config
```
"max_price_spread_threshold": 0.20,
"slippage": 0.005,
"min_quote_margin": 1,

```
```
"min_base_token_balance": 0.01
"min_quote_token_balance": 10.0
```

## Stats
### All LPs
arb.2024-06-26T165644 - 5385 rows total.

| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
|0|1429|1741|
|0.5|50|135.44|
|1|37|118.336|
|2|32|104.442|
|3|7|21.0115|

### Raydium only
arb.2024-06-26T174238 - 3208 rows total.

| Min SOL | # Arbs | Quote Token Margin |
|---------|--------|--------------------|
|0|129|265.082|
|2|27|115.625|
|3|4|19.9689|

### Raydium only, disabled CwF4aUPjMciM3u9DpxoZyLGVKVxgUgBmxHW32RbtmZNz
arb.2024-06-26T180210 - 161 rows total.

| Min SOL | # Arbs | Quote Token Margin |
|---------|--------|--------------------|
|0|55|313.383|
|2|36|270.809|
|3|10|139.061|

### Raydium only, disabled CwF4aUPjMciM3u9DpxoZyLGVKVxgUgBmxHW32RbtmZNz and FyqYBBJ8vhr5AtDZiyJue4Khx9Be6Xijx5nm6aL6wZZV
arb.2024-06-26T181920 -  225 rows total. Better results comparing to the prior run with only CwF4aUPjMciM3u9DpxoZyLGVKVxgUgBmxHW32RbtmZNz disabled!

| Min SOL | # Arbs | Quote Token Margin |
|---------|--------|--------------------|
|2|81|442.893|
|3|25|186.007|

### Raydium only two most funded AMMs.
arb.2024-06-26T193737 - 5 rows total.

| Min SOL | # Arbs | Quote Token Margin |
|---------|--------|--------------------|
|3|5|35.4236|


## Arb config
```
"min_base_token_balance": 0.5, <- increased from 0.01
"min_quote_token_balance": 10.0
"min_quote_margin": 1,
```

## Stats for Raydium only
arb.2024-06-26T203216 - 139 rows total.

| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
|0|50|299.881|
|2|36|268.691|
|3|8|132.584|

## Arb config
```
"min_base_token_balance": 0.5, <- increased from 0.01
"min_quote_token_balance": 10.0
"min_quote_margin": 1,
```

## Stats for Raydium only
arb.2024-06-26T205023 - 143 rows total. Almost same as with the 0.5 min_base_token_balance, which is surprizing.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
|0|53|307.122|
|2|36|269.988|
|3|8|132.584|

## Arb config
```
"min_base_token_balance": 0.5, <- increased from 0.01
"min_quote_token_balance": 10.0
"min_quote_margin": 1,
```

## Stats for Raydium only
arb.2024-06-26T210702 - 161 rows total. Almost same as with the 0.5 and 0.2 min_base_token_balance. Number of arbs is almost two times less that with min_base_token_balance = 0.01, but the total margin is better.

| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
|0|55|311.241|
|2|36|270.42|
|3|8|132.584|

## Arb config, Raydium only.
```
"min_base_token_balance": 0.5,
"min_quote_token_balance": 300,
"min_quote_margin": 1,
```
arb.2024-07-14T143853 - 141 rows total.

| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||54|195.348|
|1|33|130.003|
|2|33|130.003|
|3|6|40.2181|

## Arb config, Raydium only.
```
"min_base_token_balance": 1,
"min_quote_token_balance": 150,
"min_quote_margin": 1,
```
arb.2024-07-14T151409 - 203 rows total.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||72|313.462|
|2|25|154.473|
|3|21|92.3637|

## Increased Token Funding, Raydium only
```
  "token_funding": [
    {
      "code": "wSOL",
      "amount": 10.0
    },
    {
      "code": "USDC",
      "amount": 1000.0
    },
    {
      "code": "USDT",
      "amount": 1000.0
    }
  ],
```
arb.2024-07-14T152952 - 203 rows total, same as before increasing token funding. Increased quote margin.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||72|353.474|
|2|25|194.485|
|3|21|132.376|
|4|5|70.8471|
|5|5|70.8471|
|6|5|70.8471|
|7|1|17.3941|

## Decreased the Arbmin_quote_margin and base_token_balance. Raydium only.
```
"min_base_token_balance": 0.5,
"min_quote_token_balance": 300,
"min_quote_margin": 0.5,
```
arb.2024-07-14T154819 - 255 rows total.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||93|379.173|
|2|43|245.639|
|3|27|137.237|
|4|7|77.6675|
|5|6|71.6902|
|6|6|71.6902|
|7|1|17.3941|

## Decreased the Arbmin_quote_margin and base_token_balance. Raydium only.
```
"min_base_token_balance": 0.5,
"min_quote_token_balance": 300,
"min_quote_margin": 0.3,
```
arb.2024-07-14T160324 - 288 rows total. Number of arbs is a bit greater, but the total margin is not much better.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||104|383.881|
|2|48|247.775|
|3|30|138.45|
|4|7|77.6675|
|5|6|71.6902|
|6|6|71.6902|
|7|1|17.3941|


## Arb config update. Raydium only.
```
"min_base_token_balance": 0.5,
"min_quote_token_balance": 75.0,
"min_quote_margin": 0.5,
```
arb.2024-07-14T165542 - 110 rows total.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||43|273.044|
|1|40|269.392|
|2|40|269.392|
|3|7|78.5503|
|4|7|78.5503|
|5|3|56.1503|
|6|3|56.1503|
|7|1|17.3941|

## Arb config update. Raydium only.
```
"min_base_token_balance": 1,
"min_quote_token_balance": 150,
"min_quote_margin": 0.5,
```
arb.2024-07-14T172423 - 255 rows total.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||93|368.228|
|2|32|199.258|
|3|27|136.562|
|4|6|71.6902|
|5|6|71.6902|
|6|6|71.6902|
|7|1|17.3941|

## Arb config update. Raydium only.
```
"min_base_token_balance": 1,
"min_quote_token_balance": 150,
"min_quote_margin": 1,
```
arb.2024-07-14T180244 - 203 rows total.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||72|353.474|
|2|25|194.485|
|3|21|132.376|
|4|5|70.8471|
|5|5|70.8471|
|6|5|70.8471|
|7|1|17.3941|

## Arb config update. Raydium only.
```
"min_base_token_balance": 1,
"min_quote_token_balance": 150,
"min_quote_margin": 0.3,
```
arb.2024-07-14T181857 - 288 rows.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||104|372.742|
|2|35|200.472|
|3|30|137.775|
|4|6|71.6902|
|5|6|71.6902|
|6|6|71.6902|
|7|1|17.3941|

## Arb config update. Raydium only.
```
"slippage": 0.001,
"min_quote_margin": 0.1,
```
arb.2024-07-21T200151 - 1331 rows.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||390|1031.24|
|2|141|549.623|
|3|136|475.762|
|4|48|191.195|
|5|25|164.562|
|6|25|164.562|
|7|7|39.4855|

## Arb config update. Raydium only.
```
"slippage": 0.005,
```
arb.2024-07-21T201833 - 309 rows total.
| Min SOL | Qty Arbs | Quote Token Margin |
|---------|--------|--------------------|
||116|374.797|
|2|37|200.79|
|3|32|138.094|
|4|6|71.6185|
|5|6|71.6185|
|6|6|71.6185|
|7|1|17.3767|


