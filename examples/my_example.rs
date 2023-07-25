// Import the library crate
extern crate orderbook;
use orderbook::{buy_order, sell_order, initialize_trade, Trade, MenuBuyer, MenuSeller};

fn main(){
    //border和sorder分别是存储所有买方开价的向量和卖方开价的向量
    let mut border: Vec<MenuBuyer>=Vec::new();
    let mut sorder: Vec<MenuSeller>=Vec::new();

    // Buy and Sell orders added to the order book
    buy_order(&mut border, 1, 1.0, 5);
    buy_order(&mut border, 2, 2.0, 3);
    sell_order(&mut sorder, 1, 0.8, 3);
    sell_order(&mut sorder, 2, 2.1, 4);

    // Trade matching
    for buy_order in &border {
        for sell_order in &sorder {
            if buy_order.id == sell_order.id {
                if let Some(trade) = initialize_trade(buy_order, sell_order) {
                    // Perform the trade operation using the trade data
                    println!(
                        "Trade initiated: Price: {}, Quantity: {}",
                        trade.unit_price, trade.quantity
                    );
                } else {
                    println!("No matching trade found for ID: {}", buy_order.id);
                }
            }
        }
    }
}