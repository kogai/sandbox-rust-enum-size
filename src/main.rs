enum E1 {
    A(u8),
}

enum E2 {
    A(u8),
    B(u16),
}

enum E3 {
    A(u16),
}

enum E4 {
    B(u16),
    A(u8),
}

enum E5 {
    A,
    B,
}

#[repr(C)]
union E6 {
    a: u8,
    b: u16,
}

fn main() {
    println!("Size is {}", std::mem::size_of::<E1>());
    println!("Size is {}", std::mem::size_of::<E2>());
    println!("Size is {}", std::mem::size_of::<E3>());
    println!("Size is {}", std::mem::size_of::<E4>());
    println!("Size is {}", std::mem::size_of::<E5>());
    println!("Size is {}", std::mem::size_of::<E6>());
    let a = E6 { a: 1 };
    // Size is 1
    // Size is 4
    // Size is 2
    // Size is 4
    // Size is 1
    // Size is 2
}
