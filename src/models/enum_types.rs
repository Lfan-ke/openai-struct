#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    Number(isize),
    String(String),
}

pub type IntegerOrString = StringOrNumber;
