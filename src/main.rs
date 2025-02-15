use enpc::Trit::*;
use enpc::*;
fn main() {
    let my_tryte = T3::new([Two, Two, Zero]);
    println!("{my_tryte:3b}");
    let shifted_tryte = my_tryte << 1;
    println!("{shifted_tryte:3b}");
    let shifted_tryte = my_tryte << 2;
    println!("{shifted_tryte:3b}\n");

    println!("{my_tryte}");

    let my_tryte_two = T3::from(10);
    println!("{my_tryte_two}\n");

    println!("{}", Two & One);
    println!("{}", my_tryte & my_tryte_two);
}
