const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const Config = require("./config");

async function main() {
    const wsProvider = new WsProvider(Config.endPoint);
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: Config.types
    });

    const keyring = new Keyring({ type: 'sr25519' });
    const dave = keyring.addFromUri('//Dave', { name: 'Dave default' });
    const eve = keyring.addFromUri('//Eve', { name: 'Eve default' });

    const daveAccountId = "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"; // DAVE
    const eveAccountId = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // EVE

    // Dave issue symbol
    let nonce = (await api.query.system.accountNonce(daveAccountId)).toNumber();
    const [baseResult, quoteResult] = await Promise.all([
        new Promise(resolve => {
            api.tx.tokenModule.issue("USDT", 6000000).signAndSend(dave, { nonce }, (result) => {
                if (result.status.isFinalized) {
                    const record = result.findRecord("tokenModule", "Issued");
                    if (record) { resolve(record); }
                }
            });
        }),
        new Promise(resolve => {
            api.tx.tokenModule.issue("ETH", 6000000).signAndSend(dave, {nonce: nonce + 1}, (result) => {
                if (result.status.isFinalized) {
                    const record = result.findRecord("tokenModule", "Issued");
                    if (record) { resolve(record); }
                }
            });
        }),
    ])
    let baseEvent = baseResult.toJSON().event.data; // Issued(AccountId, Hash, Balance)
    let quoteEvent = quoteResult.toJSON().event.data;
    console.log("Dave issue base event: ", baseEvent);
    console.log("Dave issue quote event: ", quoteEvent);

    // Dave transfer to Eve
    const transferRecord = await new Promise(resolve => { // Transferd(AccountId, AccountId, Hash, Balance)
        api.tx.tokenModule
        .transfer(baseEvent[1], eveAccountId, 100000)
        .signAndSend(dave, (result) => {
            if (result.status.isFinalized) {
                const record = result.findRecord("tokenModule", "Transferd");
                if (record) { resolve(record); }
            }
        })
    })
    const transferEvent = transferRecord.toJSON().event.data;
    console.log("Dave transfer to Eve event: ", transferEvent);

    // Dave create tradePair
    const pairRecord = await new Promise(resolve => { // TradePairCreated(AccountId, Hash, TradePair),
        api.tx.tradeModule
        .createTradePair(baseEvent[1], quoteEvent[1])
        .signAndSend(dave, (result) => {
            if (result.status.isFinalized) {
                const record = result.findRecord("tradeModule", "TradePairCreated");
                if (record) { resolve(record); }
            }
        })
    });
    const pairEvent = pairRecord.toJSON().event.data;
    console.log("Dave create tradePair event: ", pairEvent);

    // Dave create limit order
    const orderRecord = await new Promise(resolve => { // OrderCreated (accountId, baseTokenHash, quoteTokenHash, orderHash, LimitOrder)
        api.tx.tradeModule
        .createLimitOrder(baseEvent[1], quoteEvent[1], 1, 10 * 10 ** 8, 100) // price need * 10 ** 8
        .signAndSend(dave, (result) => {
            if (result.status.isFinalized) {
                const record = result.findRecord("tradeModule", "OrderCreated");
                if (record) { resolve(record); }
            }
        })
    });

    const orderEvent = orderRecord.toJSON().event.data;
    console.log("Dave create limit order event: ", orderEvent);

    // Eve create limit order
    let records = []
    const order2Record = await new Promise(resolve => { // TradeCreated (accountId, baseTokenHash, quoteTokenHash, tradeHash, Trade)
        api.tx.tradeModule
        .createLimitOrder(baseEvent[1], quoteEvent[1], 0, 10 * 10 ** 8, 500) // when order is buy type, realamount = price * amount, price need * 10 ** 8
        .signAndSend(eve, (result) => {
            if (result.status.isFinalized) {
                const record1 = result.findRecord("tradeModule", "OrderCreated");
                const record2 = result.findRecord("tradeModule", "TradeCreated");
                if (record1) {
                    records.push(record1);
                }
                if (record2) {
                    records.push(record2);
                }
                if (records.length == 2) { resolve(records); }
            }
        })
    });
    const order2Event = order2Record[0].toJSON().event.data;
    const tradeEvent = order2Record[1].toJSON().event.data;
    console.log("Eve create limit order event: ", order2Event);
    console.log("trade created event: ", tradeEvent);

    // Dave cancel limit order
    const cancelOrderRecord = await new Promise(resolve => { // OrderCanceled(accountId, orderHash,
        api.tx.tradeModule
        .cancelLimitOrder(orderEvent[3]) // price need * 10 ** 8
        .signAndSend(dave, (result) => {
            if (result.status.isFinalized) {
                const record = result.findRecord("tradeModule", "OrderCanceled");
                if (record) { resolve(record); }
            }
        })
    });

    const cancelOrderEvent = cancelOrderRecord.toJSON().event.data;
    console.log("Dave cancel limit order event: ", cancelOrderEvent);
}

main().catch(console.error).finally(() => process.exit());
