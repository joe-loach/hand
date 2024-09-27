use std::{collections::HashMap, path::Path};

#[derive(Debug, serde::Deserialize)]
pub struct Variant {
    pub title: String,
    pub description: String,
    #[serde(with = "one_or_many")]
    pub syntax: Vec<String>,
    pub encoding: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Instuction {
    pub name: String,
    variant: Vec<Variant>,
}

impl Instuction {
    #[inline]
    #[must_use]
    pub fn variants(&self) -> &[Variant] {
        &self.variant
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct InstMap {
    instruction: HashMap<String, Instuction>,
}

impl InstMap {
    #[inline]
    pub fn keys(&self) -> std::collections::hash_map::Keys<String, Instuction> {
        self.instruction.keys()
    }

    #[inline]
    #[must_use]
    pub fn instruction(&self, s: &str) -> Option<&Instuction> {
        self.instruction.get(s)
    }

    pub(crate) fn merge(&mut self, other: InstMap) {
        self.instruction.extend(other.instruction);
    }
}

pub fn discover_all<P: AsRef<Path>>(path: P) -> anyhow::Result<InstMap> {
    fn toml_file(e: &walkdir::DirEntry) -> bool {
        e.file_type().is_file() && e.path().extension().is_some_and(|ex| ex == "toml")
    }

    let files = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(toml_file);

    let mut bin = InstMap::default();

    for entry in files {
        let s = std::fs::read_to_string(entry.path())?;
        let new: InstMap = toml::from_str(&s)?;
        bin.merge(new);
    }

    Ok(bin)
}

mod one_or_many {
    use serde::Deserialize as _;

    #[derive(Clone, Debug, serde::Deserialize, PartialEq)]
    #[serde(untagged)]
    enum OneOrMany<T> {
        One(T),
        Vec(Vec<T>),
    }

    impl<T> From<OneOrMany<T>> for Vec<T> {
        fn from(from: OneOrMany<T>) -> Self {
            match from {
                OneOrMany::One(val) => vec![val],
                OneOrMany::Vec(vec) => vec,
            }
        }
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::from(OneOrMany::deserialize(deserializer)?))
    }
}

#[test]
fn works() {
    let bin = discover_all("../../instructions").unwrap();
    for inst in bin.instruction {
        let (_name, val) = inst;
        println!("name: {} ({} variants)", val.name, val.variant.len());
    }
}
