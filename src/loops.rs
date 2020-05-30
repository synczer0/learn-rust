
#[allow(dead_code)]
pub fn for_loop() {

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
  
        println!("the value is: {}", element);
    
    }

    for number in (1..4).rev() {
        println!("{}!", number);
        }
        println!("LIFTOFF!!!");
}

#[allow(dead_code)]
fn while_loop() {

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {

        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    
}

#[allow(dead_code)]
fn loops() {

    loop {
        println!("Run Forver");
    }

}

