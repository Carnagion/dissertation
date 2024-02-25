use std::{marker::PhantomData, time::Duration};

use serde::{Deserializer, Serializer};

use serde_with::{
    formats::{Format, Strict, Strictness},
    DeserializeAs,
    DurationSeconds,
    SerializeAs,
};

use crate::sep::Separations;

// NOTE: This is so that we can store durations as minutes rather than seconds
//       in instance files, since all flights will have duration data in the
//       minutes, not seconds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DurationMinutes<Fmt = u64, Strt = Strict>(PhantomData<(Fmt, Strt)>);

impl<'de, Fmt, Strt> DeserializeAs<'de, Duration> for DurationMinutes<Fmt, Strt>
where
    Fmt: Format,
    Strt: Strictness,
    DurationSeconds<Fmt, Strt>: DeserializeAs<'de, Duration>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationSeconds::<Fmt, Strt>::deserialize_as(deserializer).map(|dur| dur * 60)
    }
}

impl<Fmt, Strt> SerializeAs<Duration> for DurationMinutes<Fmt, Strt>
where
    Fmt: Format,
    Strt: Strictness,
    DurationSeconds<Fmt, Strt>: SerializeAs<Duration>,
{
    fn serialize_as<S>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DurationSeconds::<Fmt, Strt>::serialize_as(&(*dur / 60), serializer)
    }
}

// NOTE: This is so that we can store separations in minutes rather than seconds
//       in instance files, since all flights will have separation data in the
//       minutes, not seconds.
pub struct SeparationsAsMinutes<Fmt = u64, Strt = Strict>(PhantomData<(Fmt, Strt)>);

impl<'de, Fmt, Strt> DeserializeAs<'de, Separations> for SeparationsAsMinutes<Fmt, Strt>
where
    Fmt: Format,
    Strt: Strictness,
    DurationMinutes<Fmt, Strt>: DeserializeAs<'de, Duration>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Separations, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<Vec<DurationMinutes<Fmt, Strt>>>::deserialize_as(deserializer).and_then(|grid| {
            let separations = Separations::try_from(grid).map_err(serde::de::Error::custom)?;
            Ok(separations)
        })
    }
}

impl<Fmt, Strt> SerializeAs<Separations> for SeparationsAsMinutes<Fmt, Strt>
where
    Fmt: Format,
    Strt: Strictness,
    DurationMinutes<Fmt, Strt>: SerializeAs<Duration>,
{
    fn serialize_as<S>(separations: &Separations, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let grid = separations.to_grid();
        Vec::<Vec<DurationMinutes<Fmt, Strt>>>::serialize_as(&grid, serializer)
    }
}
