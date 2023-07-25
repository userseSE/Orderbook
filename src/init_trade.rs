use crate::buy::MenuBuyer;
use crate::sale::MenuSeller;
//成交价格
pub struct Trade{
    pub id: u64,
    pub unit_price: f32,
    pub quantity: u64,
}
//成交
pub fn initialize_trade(buy_order: &MenuBuyer, sell_order: &MenuSeller)->Option<Trade>{
    if buy_order.unit_price >=sell_order.unit_price{
        let trade_quantity=buy_order.quantity.min(sell_order.quantity);
        //取双方数量最小值为成交量
        Some(Trade{
            id:buy_order.id,
            unit_price:sell_order.unit_price,
            quantity:trade_quantity,
        })
    }else{
        None
    }
}