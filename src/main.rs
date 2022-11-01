use rjvm::Result;

const FILE: &[u8] = include_bytes!("../Main.class");

fn try_main() -> Result<String>
{
    let class = rjvm::Class::parse_bytes(FILE)?;

    Ok(format!("{class:?}"))
}

fn main()
{
    match try_main() {
        Ok(res) => println!("{res:?}"),
        Err(err) => panic!("{err:?}"),
    }
}
