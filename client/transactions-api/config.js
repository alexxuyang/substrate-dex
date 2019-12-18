/* eslint-disable @typescript-eslint/camelcase */
module.exports = {
  endPoint: "ws://47.100.239.204:9944/",
  types: {
    Token: {
      hash: "H256",
      symbol: "Vec<u8>",
      total_supply: "Balance"
    },
    OrderType: {
      _enum: ["Buy", "Sell"]
    },
    OrderStatus: {
      _enum: ["Created", "PartialFilled", "Filled", "Canceled"]
    },
    TradePair: {
      hash: "H256",
      base: "H256",
      quote: "H256",
      latest_matched_price: "Option<Price>",
      one_day_trade_volume: "Balance",
      one_day_highest_price: "Option<Price>",
      one_day_lowest_price: "Option<Price>"
    },
    Price: "u128",
    LimitOrder: {
      hash: "H256",
      base: "H256",
      quote: "H256",
      owner: "AccountId",
      price: "Price",
      sell_amount: "Balance",
      buy_amount: "Balance",
      remained_sell_amount: "Balance",
      remained_buy_amount: "Balance",
      otype: "OrderType",
      status: "OrderStatus"
    },
    Trade: {
      hash: "H256",
      base: "H256",
      quote: "H256",
      buyer: "AccountId",
      seller: "AccountId",
      maker: "AccountId",
      taker: "AccountId",
      otype: "OrderType",
      price: "Price",
      base_amount: "Balance",
      quote_amount: "Balance"
    },
    OrderLinkedItem: {
      prev: "Option<Price>",
      next: "Option<Price>",
      price: "Option<Price>",
      buy_amount: "Balance",
      sell_amount: "Balance",
      orders: "Vec<H256>"
    }
  }
};
