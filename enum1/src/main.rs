#[derive(Debug)]
enum Move {
    Walk, //variants
    Jump, //variants
    Hop, //variants
    Run //variants
}
#[derive(Debug)]
enum Sweet {
    Chocolate(String), //variants
    Sweet(String), //variants
    HomeSweet(String), //variants
    Open (String,u32)//variants
}

impl Move {
    //method definition
    fn ppt(&self){
        println!("we got {:?}", self);
    }
}
//function definition
fn daily(data:Move){
    println!("data {:?}", data);

    match data {
        Walk => {println!("Walk is necessary");
                println!("Daily 40 min");}
        Jump => {println!("you are burning more calroies");}
        Hop  => {println!("you are burning good calroies");}
        Run  => {println!("you are burning highest calroies");}
    }
 }

fn main() {
    let mymove = Move::Walk;
    println!("{:?}",mymove);
    let urmove = Move::Run;
    println!("{:?}",urmove);
    let hermove = Move::Jump;
    println!("{:?}",hermove);
    let hismove = Move::Run;
    println!("{:?}",hismove);
    daily(urmove); //function calling
    daily(hermove);
 
    mymove.ppt(); //method calling

    let mySweet = Sweet::Chocolate(String::from("Rocher"));
    let urSweet = Sweet::Open(String::from("Mars"),1);
    println!("My sweet {:?}",mySweet);
}
