

fn main() {

    let data_type_001 = ("abc", 'a', 0);

    println!("{}", data_type_001.0);
    println!("{}", data_type_001.1);
    println!("{}", data_type_001.2);

    let data_type_002 = (1i32, false);
    println!("{}", data_type_002.0);
    println!("{}", data_type_002.1);

    let data_type_003 = (1i32, (true, 2i128));

    println!("{}", data_type_003.0);
    println!("{}", data_type_003.1.0);
    println!("{}", data_type_003.1.1);

    println!("size of i8 {}", std::mem::size_of::<i8>());
    println!("size of char {}", std::mem::size_of::<char>());
    println!("size of () {}", std::mem::size_of::<()>());
}


