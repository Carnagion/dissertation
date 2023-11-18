use std::{marker::PhantomData, time::Duration};

use serde::{Deserializer, Serializer};

use serde_with::{
    formats::{Flexible, Format, Strict, Strictness},
    DeserializeAs, DurationSeconds, SerializeAs,
};

// NOTE: This is so that we can store durations as minutes rather than seconds in instance CSV files,
// since realistically most aircraft will have separations in the minutes and not seconds
pub struct DurationMinutes<FORMAT = u64, STRICTNESS = Strict>(PhantomData<(FORMAT, STRICTNESS)>)
where
    FORMAT: Format,
    STRICTNESS: Strictness;

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

impl<'de, FORMAT> DeserializeAs<'de, Duration> for DurationMinutes<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationSeconds::<FORMAT, Flexible>::deserialize_as(deserializer)
            .map(|dur: Duration| dur * 60)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationMinutes<u64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<u64, STRICTNESS>::serialize_as(&(*source / 60), serializer)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationMinutes<f64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<f64, STRICTNESS>::serialize_as(&(*source / 60), serializer)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationMinutes<String, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<String, STRICTNESS>::serialize_as(&(*source / 60), serializer)
    }
}
