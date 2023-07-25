//Buyer开价
pub struct MenuBuyer {
    pub id: u64,
    pub good: String,
    pub unit_price: f32,
    pub quantity: u64,
}

pub fn buy_order(order_power: &mut Vec<MenuBuyer>, id: u64, good: String, unit_price: f32, quantity: u64) {
    let order = MenuBuyer { id, good, unit_price, quantity };
    order_power.push(order);
}