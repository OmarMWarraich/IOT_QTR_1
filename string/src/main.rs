fn main() {
    let fname = String::from("Omar");
    let mname = String::from("Mohammad");
    let lname = String::from("Warraich");
    // let name = fname + &mname + &lname ;
    let name = format!("{} {} {}",fname,mname,lname) ;
    println!("Complete name {}",name);
    println!("First name {}",fname);
    println!("Middle name {}",mname);
    println!("Last name {}",lname);
    
    let country = String::from("Pakistan");

    let charachter = &country[0..1];
    let somecharachter = &country[0..=6];
    println!("{}",country);
    println!("{}",charachter);
    println!("{}",somecharachter);
    
}
