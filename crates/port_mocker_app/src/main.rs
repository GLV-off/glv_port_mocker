use log::info;

const LOG_CFG: &str = "log.yaml";

pub fn init_log() -> anyhow::Result<()> {
    match log4rs::init_file(LOG_CFG, Default::default()) {
        Ok(_) => info!("todo: application under dev"),
        Err(error) => println!("ERROR: loging initialization failed: filepath = \"{}\", {}", LOG_CFG, error)
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_log()?;
    Ok(())
}
