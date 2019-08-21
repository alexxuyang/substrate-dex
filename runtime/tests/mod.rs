use parity_codec::{Compact, Encode};

#[derive(Encode)]
pub enum OrderType {
	Buy,
	Sell,
}

#[test]
fn test_enum() {
    let a = OrderType::Buy;
    a.using_encoded(|x| {
        println!("{:?}", x);
    });
}
