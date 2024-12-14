use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::SerializeStruct,
    Serialize,
};

use crate::support;

/// A request envelope that includes the type name of the given type. Wrap your
/// structs in this to get type tagged structures with data contents.
#[derive(Clone, Debug)]
pub struct Request<T>(pub T);

impl<T> Serialize for Request<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = serializer.serialize_struct("Request", 2)?;
        serializer.serialize_field(
            "type",
            &support::type_name(&self.0).map_err(|_| serde::ser::Error::custom("not struct"))?,
        )?;
        serializer.serialize_field("data", &self.0)?;
        serializer.end()
    }
}

impl<T> Request<T> {
    /// Create a new request envelope with the given data.
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl<'de, Data> Deserialize<'de> for Request<Data>
where
    Data: Deserialize<'de> + Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RequestVisitor<Data> {
            marker: std::marker::PhantomData<Data>,
        }

        impl<'de, Data> Visitor<'de> for RequestVisitor<Data>
        where
            Data: Deserialize<'de> + Serialize,
        {
            type Value = Request<Data>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Request")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut given_type: Option<String> = None;
                let mut data = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "type" => {
                            given_type = Some(map.next_value()?);
                        }
                        "data" => {
                            data = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let given_type = given_type.ok_or_else(|| de::Error::missing_field("type"))?;

                let expected_type =
                    support::type_name(&data).map_err(|_| de::Error::custom("not struct"))?;

                if expected_type != given_type {
                    return Err(de::Error::custom(format!(
                        "wrong type: expected {expected_type}, got {given_type}"
                    )));
                }
                Ok(Request::new(data))
            }
        }

        deserializer.deserialize_struct(
            "Request",
            &["type", "data"],
            RequestVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}
