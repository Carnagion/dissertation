use std::{marker::PhantomData, time::Duration};

use serde::{Deserializer, Serializer};

use serde_with::{
    formats::{Flexible, Format, Strict, Strictness},
    DeserializeAs,
    DurationSeconds,
    SerializeAs,
};

// NOTE: This is so that we can store durations as minutes rather than seconds
//       in instance CSV files, since realistically most aircraft will have
//       separations in the minutes and not seconds
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DurationMinutes<Fmt = u64, Strt = Strict>(PhantomData<(Fmt, Strt)>)
where
    Fmt: Format,
    Strt: Strictness;

impl<'de> DeserializeAs<'de, Duration> for DurationMinutes<u64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationSeconds::<u64, Strict>::deserialize_as(deserializer).map(|dur: Duration| dur * 60)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationMinutes<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationSeconds::<f64, Strict>::deserialize_as(deserializer).map(|dur: Duration| dur * 60)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationMinutes<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationSeconds::<String, Strict>::deserialize_as(deserializer)
            .map(|dur: Duration| dur * 60)
    }
}

impl<'de, Fmt> DeserializeAs<'de, Duration> for DurationMinutes<Fmt, Flexible>
where
    Fmt: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationSeconds::<Fmt, Flexible>::deserialize_as(deserializer).map(|dur: Duration| dur * 60)
    }
}

impl<Strt> SerializeAs<Duration> for DurationMinutes<u64, Strt>
where
    Strt: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<u64, Strt>::serialize_as(&(*source / 60), serializer)
    }
}

impl<Strt> SerializeAs<Duration> for DurationMinutes<f64, Strt>
where
    Strt: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<f64, Strt>::serialize_as(&(*source / 60), serializer)
    }
}

impl<Strt> SerializeAs<Duration> for DurationMinutes<String, Strt>
where
    Strt: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<String, Strt>::serialize_as(&(*source / 60), serializer)
    }
}
