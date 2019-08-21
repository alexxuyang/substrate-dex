use parity_codec::{Compact, Encode};

#[derive(Encode)]
pub enum OrderType {
	Buy,
	Sell,
}
fn main() {
    let a = OrderType::Buy;
    let b = OrderType::Sell;

    a.using_encoded(|x| {
        println!("{:?}", x);
    });

    b.using_encoded(|x| {
        println!("{:?}", x);
    });
}