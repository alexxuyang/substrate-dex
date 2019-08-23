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

    let f1 = 1.1f64 + 2.2f64 + 0.2f64;
    let f2 = 0.8f64 + 2.1f64 + 0.6f64;
    let f3 = 3.3f64 + 0.2f64;
    let f4 = 3.3f64 + 0.1f64 + 0.1f64;
    let f5 = 3.5f64;
    println!("{}, {}, {}, {}, {}", f1, f2, f3, f4, f5);
}