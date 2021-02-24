#[derive(Debug)]
enum Move {
    Walk,
    Jump,
    Hop,
    Run,
}
#[derive(Debug)]
enum Sweet {
    Chocolate(String),
    Sweet(String),
    HomeSweet(String),
    Open(String,u32 ),
}
impl Move {
    fn ppt(&self){
        println!("we got {:?}",self);
    }
}

fn daily(data:Move) {
    println!("data {:?}",data);


}
fn main() {
    let mymove = Move::Walk;
    println!("{:?}", mymove);
    let urmove = Move::Run;
    println!("{:?}", urmove);
    daily(urmove);
    mymove.ppt();
    let mysweet = Sweet::Chocolate(String::from("Rocher"));
    let ursweet = Sweet::Open(String::from("Twix"),1);
    println!("{:?}", mysweet);
    println!("{:?}", ursweet);
}
