fn main() {
    let str1 = "Hello, world!";
    println!("&str1={:?}", str1);

    //&str=>String
    let string1 = String::from(str1);
    println!("String1={:?}", string1);
    let string2 = str1.to_string();
    println!("String2={:?}", string2);

    //&str=>&[u8]
    let u81 = str1.as_bytes();
    println!("&[u8]1={:?}", u81);

    //&str=>&[u8]=>Vec[u8]
    let vecu81 = str1.as_bytes().to_vec();
    println!("Vec<u8>1={:?}", vecu81);
    let vecu82 = str1.as_bytes().to_owned();
    println!("Vec<u8>2={:?}", vecu82);

    //String=>&str
    let string1 = String::from(str1);
    let str2 = string1.as_str();
    println!("&str2={:?}", str2);

    //String=>&[u8]
    let string1 = String::from(str1);
    let u82 = string1.as_bytes();
    println!("&[u8]2={:?}", u82);

    //String=>Vec[u8]
    let string2 = String::from("test2");
    let vecu85 = string2.into_bytes();
    println!("Vec<u8>5={:?}", vecu85);

    //[u8]=>Vec[u8]
    let u82 =[72_u8, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
    // let vecu83 = u82.to_owned();
    // println!("Vec<u8>3={:?}", vecu83);
    let vecu84 = u82.to_vec();
    println!("Vec<u8>4={:?}", vecu84);

    //&[u8]=>&str
    let u83 =[72_u8, 101, 108, 108, 111, 44, 32, 119];
    let str3 = std::str::from_utf8(&u83).unwrap();
    println!("str3={:?}", str3);

    //&[u8]=>Vec[u8]=>String
    let u83 =[72_u8, 101, 108, 108, 111, 44, 32, 119,0];
    let string4 = String::from_utf8(u83.to_vec()).unwrap();
    println!("String4={:?}", string4);

    //Vec[u8]=>String
    let vecu84 =[72_u8, 101, 108, 108, 111, 44, 32, 119, 0, 1].to_vec();
    let string4 = String::from_utf8(vecu84).unwrap();
    println!("String4={:?}", string4);

    //Vec[u8]=>&[u8]
    let vecu84 =[72_u8, 101, 108, 108, 111, 44, 32, 119, 0, 1].to_vec();
    let u84 = vecu84.as_slice();
    println!("u84={:?}", u84);

    //Vec[u8]=>&str
    let vecu84 =[72_u8, 101, 108, 108, 111, 44, 32, 119, 0, 1].to_vec();
    let str4 =  std::str::from_utf8(&vecu84).unwrap();
    println!("str4={:?}", str4);

}


