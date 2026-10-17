fn main() {
    let coffee_price = 5.99;
    {
        println!("price: {}", coffee_price);
        let new_coffee_price = 1.0;
    }
    //println!("new_coffee_price: {}", new_coffee_price);     // error: out of the inner scope
}
