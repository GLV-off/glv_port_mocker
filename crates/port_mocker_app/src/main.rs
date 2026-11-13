use log::info;

const LOG_CFG: &str = "log.yaml";

fn main() -> anyhow::Result<()> {
    log4rs::init_file(LOG_CFG, Default::default())?;
    info!("Hello, world!");
    Ok(())
}
