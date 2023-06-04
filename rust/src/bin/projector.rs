use anyhow::Result;
use clap::Parser;
use rust::{
    config::{Config, Operation},
    opts::Opts,
    projector::Projector,
};

fn main() -> Result<()> {
    let cfg: Config = Opts::parse().try_into()?;
    let mut proj = Projector::from_config(cfg.config, cfg.pwd);
    match cfg.operation {
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value)
        }
        Operation::Print(Some(k)) => {
            proj.get_value(&k).map(|x| {
                println!("{}", x);
            });
        }
        Operation::Add(k, v) => {
            proj.set_value(&k, &v);
            proj.save()?
        }
        Operation::Delete(k) => {
            proj.del_value(&k);
            proj.save()?
        }
    }
    return Ok(());
}
