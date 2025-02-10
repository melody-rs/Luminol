use alox_48::{DeError, SerializeHash, SerializeIvars};

// Copyright (C) 2024 Melody Madeline Lyons
//
// This file is part of Luminol.
//
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.
use crate::rpg::{AudioFile, Event};
use crate::{id_alox, option_vec, Table3};

#[derive(Default, Debug)]
pub struct Map {
    pub tileset_id: usize,
    pub width: usize,
    pub height: usize,
    pub autoplay_bgm: bool,
    pub bgm: AudioFile,
    pub autoplay_bgs: bool,
    pub bgs: AudioFile,
    pub encounter_list: Vec<i32>,
    pub encounter_step: i32,
    pub data: Table3,
    pub events: option_vec::OptionVec<Event>,

    pub modified: bool,
}

pub struct MapDeserializer<'res> {
    pub command_db: &'res crate::CommandDB,
}

struct Visitor<'res> {
    command_db: &'res crate::CommandDB,
}

struct MultiEventDeserializer<'res> {
    command_db: &'res crate::CommandDB,
}

impl<'res, 'de> alox_48::de::DeserializeSeed<'de> for MultiEventDeserializer<'res> {
    type Value = option_vec::OptionVec<Event>;

    fn deserialize<D>(self, deserializer: D) -> alox_48::DeResult<Self::Value>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        struct Visitor<'res> {
            command_db: &'res crate::CommandDB,
        }
        impl<'res, 'de> alox_48::de::Visitor<'de> for Visitor<'res> {
            type Value = option_vec::OptionVec<Event>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array of RPG::Event")
            }

            fn visit_hash<A>(self, mut map: A) -> alox_48::DeResult<Self::Value>
            where
                A: alox_48::HashAccess<'de>,
            {
                let Self { command_db } = self;
                // cleanest way I could find of doing it
                std::iter::from_fn(|| {
                    let index = match map.next_key().transpose()? {
                        Ok(index) => index,
                        Err(e) => return Some(Err(e)),
                    };
                    let seed = crate::rpg::EventDeserializer { command_db };
                    let result = map.next_value_seed(seed).map(|event| (index, event));
                    Some(result)
                })
                .collect()
            }
        }

        let Self { command_db } = self;
        deserializer.deserialize(Visitor { command_db })
    }
}

impl<'res, 'de> alox_48::de::Visitor<'de> for Visitor<'res> {
    type Value = Map;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an instance of RPG::Map")
    }

    fn visit_object<A>(
        self,
        _: &'de alox_48::Sym,
        mut instance_variables: A,
    ) -> alox_48::DeResult<Self::Value>
    where
        A: alox_48::IvarAccess<'de>,
    {
        let Self { command_db } = self;
        let mut tileset_id = None;
        let mut width = None;
        let mut height = None;
        let mut autoplay_bgm = None;
        let mut bgm = None;
        let mut autoplay_bgs = None;
        let mut bgs = None;
        let mut encounter_list = None;
        let mut encounter_step = None;
        let mut data = None;
        let mut events = None;

        while let Some(field) = instance_variables.next_ivar()? {
            match field.to_rust_field_name().unwrap_or(field).as_str() {
                "tileset_id" => {
                    let id = instance_variables.next_value::<id_alox::DeserializeId>()?;
                    tileset_id = Some(id.0);
                }
                "width" => width = Some(instance_variables.next_value()?),
                "height" => height = Some(instance_variables.next_value()?),
                "autoplay_bgm" => autoplay_bgm = Some(instance_variables.next_value()?),
                "bgm" => bgm = Some(instance_variables.next_value()?),
                "autoplay_bgs" => autoplay_bgs = Some(instance_variables.next_value()?),
                "bgs" => bgs = Some(instance_variables.next_value()?),
                "encounter_list" => encounter_list = Some(instance_variables.next_value()?),
                "encounter_step" => encounter_step = Some(instance_variables.next_value()?),
                "data" => data = Some(instance_variables.next_value()?),
                "events" => {
                    let seed = MultiEventDeserializer { command_db };
                    let value = instance_variables.next_value_seed(seed)?;
                    events = Some(value);
                }
                _ => {
                    instance_variables.next_value::<alox_48::de::Ignored>()?;
                }
            }
        }

        Ok(Map {
            tileset_id: tileset_id.ok_or_else(|| DeError::missing_field("tileset_id".into()))?,
            width: width.ok_or_else(|| DeError::missing_field("width".into()))?,
            height: height.ok_or_else(|| DeError::missing_field("height".into()))?,
            autoplay_bgm: autoplay_bgm
                .ok_or_else(|| DeError::missing_field("autoplay_bgm".into()))?,
            bgm: bgm.ok_or_else(|| DeError::missing_field("bgm".into()))?,
            autoplay_bgs: autoplay_bgs
                .ok_or_else(|| DeError::missing_field("autoplay_bgs".into()))?,
            bgs: bgs.ok_or_else(|| DeError::missing_field("bgs".into()))?,
            encounter_list: encounter_list
                .ok_or_else(|| DeError::missing_field("encounter_list".into()))?,
            encounter_step: encounter_step
                .ok_or_else(|| DeError::missing_field("encounter_step".into()))?,
            data: data.ok_or_else(|| DeError::missing_field("data".into()))?,
            events: events.ok_or_else(|| DeError::missing_field("events".into()))?,
            modified: false,
        })
    }
}

impl<'res, 'de> alox_48::de::DeserializeSeed<'de> for MapDeserializer<'res> {
    type Value = Map;

    fn deserialize<D>(self, deserializer: D) -> alox_48::DeResult<Self::Value>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        let Self { command_db } = self;
        deserializer.deserialize(Visitor { command_db })
    }
}

impl<'res, 'de> serde::de::DeserializeSeed<'de> for MapDeserializer<'res> {
    type Value = Map;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

pub struct MapSerializer<'res> {
    pub command_db: &'res crate::CommandDB,
    pub map: &'res Map,
}

impl<'res> alox_48::ser::Serialize for MapSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let Self { command_db, map } = *self;
        let Map {
            tileset_id,
            width,
            height,
            autoplay_bgm,
            bgm,
            autoplay_bgs,
            bgs,
            encounter_list,
            encounter_step,
            data,
            events,
            ..
        } = map;
        let mut ivars = serializer.serialize_object("RPG::Map".into(), 11)?;

        ivars.serialize_entry("@tileset_id".into(), &id_alox::SerializeId(*tileset_id))?;
        ivars.serialize_entry("@width".into(), width)?;
        ivars.serialize_entry("@height".into(), height)?;
        ivars.serialize_entry("@autoplay_bgm".into(), autoplay_bgm)?;
        ivars.serialize_entry("@bgm".into(), bgm)?;
        ivars.serialize_entry("@autoplay_bgs".into(), autoplay_bgs)?;
        ivars.serialize_entry("@bgs".into(), bgs)?;
        ivars.serialize_entry("@encounter_list".into(), encounter_list)?;
        ivars.serialize_entry("@encounter_step".into(), encounter_step)?;
        ivars.serialize_entry("@data".into(), data)?;
        ivars.serialize_entry(
            "@events".into(),
            &MultiEventSerializer { command_db, events },
        )?;

        ivars.end()
    }
}

struct MultiEventSerializer<'res> {
    command_db: &'res crate::CommandDB,
    events: &'res option_vec::OptionVec<Event>,
}

impl<'res> alox_48::Serialize for MultiEventSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let Self { command_db, events } = *self;
        let mut ser = serializer.serialize_hash(events.size())?;
        for (index, event) in events {
            ser.serialize_key(&index)?;
            ser.serialize_value(&crate::rpg::EventSerializer { command_db, event })?;
        }
        ser.end()
    }
}

impl<'res> serde::Serialize for MapSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
