use std::{borrow::Cow, fmt, str::FromStr};

use ::xid as kazk_xid;
use bonsaidb::core::key;
use bonsaidb::core::key::{CompositeKeyDecoder, CompositeKeyEncoder, CompositeKeyError};
use serde::de::Visitor;
use shipyard::Component;
type InlineShortString = smartstring::SmartString<smartstring::Compact>;

#[derive(Component)]
/// standard Rust equality/comparison derives
#[derive(Eq, PartialEq, Ord, Hash, PartialOrd)]
/// Goodies
#[derive(Clone)]
pub struct HintedID {
    // `xid` ordered first so xid is checked first for ordering
    xid: [u8; 12],
    prefix: InlineShortString,
}

impl key::KeyEncoding for HintedID {
    type Error = CompositeKeyError;

    const LENGTH: Option<usize> = None;

    fn describe<Visitor>(visitor: &mut Visitor)
    where
        Visitor: key::KeyVisitor,
    {
        visitor.visit_composite(key::CompositeKind::Tuple, 2);
        visitor.visit_type(key::KeyKind::Bytes);
        visitor.visit_type(key::KeyKind::String);
    }

    fn as_ord_bytes(&self) -> Result<std::borrow::Cow<'_, [u8]>, Self::Error> {
        let mut enc = CompositeKeyEncoder::default();
        // we know these cant fail
        enc.encode(&self.xid)?;
        enc.encode(&self.prefix.as_str())?;
        Ok(Cow::Owned(enc.finish()))
    }
}

impl<'k> key::Key<'k> for HintedID {
    const CAN_OWN_BYTES: bool = false;

    fn from_ord_bytes<'e>(bytes: key::ByteSource<'k, 'e>) -> Result<Self, Self::Error> {
        let mut dec = CompositeKeyDecoder::default_for(bytes);
        Ok(Self {
            xid: dec.decode()?,
            prefix: dec.decode::<String>()?.into(),
        })
    }
}

impl HintedID {
    fn parse<E: serde::de::Error>(v: &str) -> Result<Self, E> {
        let (prefix, xid_str) = v.rsplit_once('_').ok_or_else(|| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"hintedid expected to have an underscore _ between a prefix and an xid",
            )
        })?;
        Ok(HintedID {
            prefix: prefix.into(),
            xid: kazk_xid::Id::from_str(xid_str)
                .map_err(|e| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(xid_str),
                        &format!("hintedid had invalid XID component: {e:?}").as_str(),
                    )
                })?
                .0,
        })
    }
}

impl TryFrom<&str> for HintedID {
    type Error = serde::de::value::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        HintedID::parse(v)
    }
}

impl fmt::Debug for HintedID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_id_string())
    }
}

impl fmt::Display for HintedID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_id_string())
    }
}

impl HintedID {
    pub fn generate(prefix: &str) -> Self {
        HintedID {
            prefix: prefix.into(),
            xid: kazk_xid::new().0,
        }
    }

    pub fn to_id_string(&self) -> String {
        format!("{}_{}", self.prefix, &kazk_xid::Id(self.xid).to_string(),)
    }
}

struct HintedIDVisitor;

impl<'de> Visitor<'de> for HintedIDVisitor {
    type Value = HintedID;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid hinted ID string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        HintedID::parse(v)
    }
}

impl<'d> serde::Deserialize<'d> for HintedID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        deserializer.deserialize_str(HintedIDVisitor)
    }
}

impl serde::Serialize for HintedID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_id_string())
    }
}
