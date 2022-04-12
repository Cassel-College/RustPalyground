

fn main() {

    // --------------------------------------------------
    let mut variable : i32  = 100;

    println!("{}", variable);

    println!("{:#?}", funcation_001(12, 23));

    println!("{p}---{pp}", p = funcation_001(12, 23), pp = "po");

    variable = 123;
    println!("{}", variable);

    
    // --------------------------------------------------
    let num : i32;
    num = 12;
    println!("{}", num);

    // --------------------------------------------------

    let num1 : i8 = 120;
    let num2 : i8 = 120;
    println!("120 + 120 = {}", num1 as i16 + num2 as i16)
}


fn funcation_001(num_1 : i32, num_2 : i32) -> i32{
    num_1 + num_2
}
