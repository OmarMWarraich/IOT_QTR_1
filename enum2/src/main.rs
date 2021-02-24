#[derive(Debug)]
enum Foode {
    Rooti(u8),
    Pizza(String),
    Fastfood(String),

}

impl Foode {
    fn ppt(&self){
        println!("{:#?}",self);
    }
}
#[derive(Debug)]
struct Foods { //fields
    Rooti:String,
    Pizza:String,
    Fastfood:String,

}
fn main() {
    let e1 = Foode::Rooti(4);
    println!("{:#?}",e1);
    e1.ppt();
    let s1 = Foods {
        Rooti : String::from("Garlic Nan"),
        Pizza: String::from("Chicken fajeeta"),
        Fastfood: String::from("Nali Nehari"),
    };
    println!("{:#?}",s1);

    println!("Hello, world!");
}
