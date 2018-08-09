enum E1 {
    DblVal1(f64),
}

enum E2 {
    DblVal1(f64),
    DblVal2(f64),
    DblVal3(f64),
    DblVal4(f64),
}

enum E3 {
    A(u8),
    B(u16),
    C(u32, u32, u32),
}

enum E4 {
    B(u16),
    C(u32, u32, u32),
    A(u8),
}

fn main() {
    println!("Size is {}", std::mem::size_of::<E1>());
    println!("Size is {}", std::mem::size_of::<E2>());
    println!("Size is {}", std::mem::size_of::<E3>());
    println!("Size is {}", std::mem::size_of::<E4>());
    // Size is 8
    // Size is 16
    // Size is 8
    // Size is 8
}
