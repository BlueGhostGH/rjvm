const FILE: &[u8] = include_bytes!("../Main.class");

fn try_main() -> rjvm::error::Result<String>
{
    let class = rjvm::parse(FILE)?;

    Ok(format!("{class:?}"))
}

fn main()
{
    match try_main() {
        Ok(res) => println!("{res}"),
        Err(err) => panic!("{err:?}"),
    }
}
