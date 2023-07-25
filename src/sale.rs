//Seller开价
pub struct MenuSeller {
    pub id: u64,
    pub good: String,
    pub unit_price: f32,
    pub quantity: u64,
}


pub fn sell_order(order_power: &mut Vec<MenuSeller>, id:u64, good:String, unit_price:f32, quantity:u64) {
    let order=MenuSeller{id, good, unit_price, quantity};
    order_power.push(order);
}
    