const FILE: &[u8] = include_bytes!("../Main.class");

fn main()
{
    let class = rjvm::Class::from_bytes(FILE);

    println!("{class:?}");
}
