use crate::opts::Opts;
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Delete(String),
}

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(opts: Opts) -> Result<Self> {
        let operation = opts.args.try_into()?;
        let pwd = get_pwd(opts.pwd)?;
        let config = get_config(opts.config)?;

        return Ok(Config {
            operation,
            pwd,
            config,
        });
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }
        let term = value.get(0).expect("Should exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!("Operation add requires 2 arguments"));
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }
        if term == "del" {
            if value.len() != 2 {
                return Err(anyhow!("Operation delete requires 1 argument"));
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Delete(arg));
        }
        if value.len() > 1 {
            return Err(anyhow!("Operation print requires 0 or 1 argument"));
        }
        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let loc = std::env::var("XDG_CONFIG_HOME").context("XDG_CONFIG_HOME not set")?;
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");
    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }
    return Ok(std::env::current_dir().context("Could not get current directory")?);
}
#[cfg(test)]
mod test {
    use crate::{opts::Opts, config::{Config, Operation}};
    use anyhow::{Result};

    #[test]
    fn test_print_all() -> Result<()>{
        let config: Config = Opts{
            pwd: None,
            args: vec![],
            config: None,
        }.try_into()?;
        assert_eq!(config.operation, Operation::Print(None));
        return Ok(())
    }

    #[test]
    fn test_print_key() -> Result<()>{
        let config: Config = Opts{
            pwd: None,
            args: vec![String::from("foo")],
            config: None,
        }.try_into()?;
        assert_eq!(config.operation, Operation::Print(Some(String::from("foo"))));
        return Ok(())
    }


    #[test]
    fn test_add() -> Result<()>{
        let config: Config = Opts{
            pwd: None,
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
            config: None,
        }.try_into()?;
        assert_eq!(config.operation, Operation::Add(String::from("foo"), String::from("bar")));
        return Ok(())
    }

    #[test]
    fn test_delete() -> Result<()>{
        let config: Config = Opts{
            pwd: None,
            args: vec![
                String::from("del"),
                String::from("foo"),
            ],
            config: None,
        }.try_into()?;
        assert_eq!(config.operation, Operation::Delete(String::from("foo")));
        return Ok(())
    }

}

