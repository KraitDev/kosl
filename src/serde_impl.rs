use serde::de::{self, Visitor, MapAccess, SeqAccess, IntoDeserializer};
use std::collections::HashMap;
use crate::value::KoslValue;

/// Entry point wrapper binding our explicit syntax parser directly to Serde target structures.
pub fn deserialize_from_str<'a, T: serde::Deserialize<'a>>(input: &'a str) -> Result<T, String> {
    let parsed_map = crate::parser::from_str(input)?;
    let root_value = KoslValue::Object(parsed_map);
    T::deserialize(root_value).map_err(|e| e.to_string())
}

struct SeqDeserializer {
    iter: std::vec::IntoIter<KoslValue>,
}

impl SeqDeserializer {
    fn new(vec: Vec<KoslValue>) -> Self {
        SeqDeserializer { iter: vec.into_iter() }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }
}

struct MapDeserializer {
    iter: std::collections::hash_map::IntoIter<String, KoslValue>,
    next_value: Option<KoslValue>,
}

impl MapDeserializer {
    fn new(map: HashMap<String, KoslValue>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            next_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.next_value = Some(value);
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.next_value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(de::Error::custom("Structural mapping failure: missing data entry")),
        }
    }
}

impl<'de> de::Deserializer<'de> for KoslValue {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            KoslValue::String(s) => visitor.visit_string(s),
            KoslValue::Int(i) => visitor.visit_i64(i),
            KoslValue::Float(f) => visitor.visit_f64(f),
            KoslValue::Bool(b) => visitor.visit_bool(b),
            KoslValue::Object(map) => visitor.visit_map(MapDeserializer::new(map)),
            KoslValue::List(list) => visitor.visit_seq(SeqDeserializer::new(list)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { visitor.visit_newtype_struct(self) }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            KoslValue::String(s) => visitor.visit_enum(s.into_deserializer()),
            _ => Err(de::Error::custom("Validation Error: Expected a valid string value for typed variant mappings")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
