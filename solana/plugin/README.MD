Reference Implementation of the [Solana Geyser Plugin](https://github.com/anza-xyz/agave/tree/master/geyser-plugin-interface) for asyncronous processing of the plugin's callbacks.

It registers callbacks that are sending messages to a single queue that is being drained by a sole reader called Procesor. This approach makes message processing thread safe and also decouples it from the Validator API, which make Processors re-usable by other than the Solana Validator application and simplifies it's testing.

Reference instructions how to build and run it.
### Obtain the source code
1. `mkdir -p ~/work/src/`
1. `cd ~/work/src`
1. `git pull git@github.com:andreimontchik/public.git`
1. `cd solana/plugin`

### Build 
`cargo build`

### Test
`cargo test`

### Create Config
* Use the [plugin.localnet.json](config/plugin.localnet.json) as reference. 

### Run on Localnet
1. `mkdir -p ~/work/playground`
1. `cd ~/work/playground`
1. `export RUST_LOG=solana=info,solana_metrics=warn,async_plugin=info; rm -f test-ledger/validator.log;  solana-test-validator --geyser-plugin-config ~/work/src/public/solana/plugin/config/plugin.localnet.json`