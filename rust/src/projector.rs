use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    data: Data,
    config: Config,
}

impl Projector {
    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;
        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            curr = p.parent()
        }
        return out;
    }

    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut paths = vec![];

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent()
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter())
            }
        }

        return out;
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        self.data
            .projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key.to_string(), value.to_string());
    }

    pub fn del_value(&mut self, key: &str) {
        self.data.projector.get_mut(&self.config.pwd).map(|x| {
            x.remove(key);
        });
    }

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = std::fs::read_to_string(&config.config);
            // different approach
            let contents = contents.unwrap_or("{\"projector\":{}".to_string());
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data::default());
            return Projector { config, data };
        }
        return Projector {
            data: Data::default(),
            config,
        };
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use crate::config::Config;

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
                PathBuf::from("/") => hashmap!{
                    "foo".into() => "bar1".into(),
                    "fem".into() => "is_great".into(),
                },
                PathBuf::from("/foo") => hashmap! {
                    "foo".into() => "bar2".into()
                },
                PathBuf::from("/foo/bar") => hashmap!{
                    "foo".into() => "bar3".into(),
            },
        };
    }

    fn get_project(pwd: PathBuf) -> Projector {
        return Projector {
            config: Config {
                pwd,
                config: PathBuf::from(""),
                operation: crate::config::Operation::Print(None),
            },
            data: Data {
                projector: get_data(),
            },
        };
    }

    #[test]
    fn get_value() {
        let proj = get_project(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")))
    }

    #[test]
    fn set_value() {
        let mut proj = get_project(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        proj.set_value("foo", "bar4");
        proj.set_value("fem", "is_better_than_great");
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar4")));
        assert_eq!(
            proj.get_value("fem"),
            Some(&String::from("is_better_than_great"))
        )
    }

    #[test]
    fn del_value() {
        let mut proj = get_project(PathBuf::from("/foo/bar"));
        proj.del_value("foo");
        proj.del_value("fem");
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar2")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")))
    }
}
