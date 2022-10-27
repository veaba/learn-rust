#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    #[serde(with = "json_string")]
    sms: Sms,
    uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Sms {
    source: u64,
    destination: u64,
    content: String,
}

mod json_string {
    use serde_json;
    use serde::ser::{self, Serialize, Serializer};
    use serde::de::{self, Deserialize, DeserializeOwned, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: Serialize,
              S: Serializer
    {
        let j = serde_json::to_string(value).map_err(ser::Error::custom)?;
        j.serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where T: DeserializeOwned,
              D: Deserializer<'de>
    {
        let j = String::deserialize(deserializer)?;
        serde_json::from_str(&j).map_err(de::Error::custom)
    }
}

fn main() {
    let msg = r#"
      {
        "sms":"{\"Source\":4477665544,\"Destination\":1231231,\"Content\":\"Hello from SMPPSim\"}",
        "uuid":"69e123f4-4ced-4f8f-9853-df20ebc3937b"
      }"#;

    let de = serde_json::from_str::<Message>(msg).unwrap();
    println!("{:#?}", de);

    let ser = serde_json::to_string_pretty(&de).unwrap();
    println!("{}", ser);
}
