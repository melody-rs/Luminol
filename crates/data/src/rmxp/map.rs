use alox_48::{DeError, SerializeIvars};
use serde::de::Error;
use serde::ser::SerializeStruct;

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
use crate::{id_alox, id_serde, option_vec, Table3};

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Map {
    #[serde(with = "id_serde")]
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

    #[serde(skip)]
    pub modified: bool,
}

pub struct MapDeserializer<'res> {
    pub command_db: &'res crate::CommandDB,
}

struct Visitor<'res> {
    command_db: &'res crate::CommandDB,
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
                    todo!();
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

enum Field {
    TilesetId,
    Width,
    Height,
    AutoplayBgm,
    Bgm,
    AutoplayBgs,
    Bgs,
    EncounterList,
    EncounterStep,
    Data,
    Events,
    Ignore,
}

struct FieldVisitor;

impl<'de> serde::de::Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("field identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(v.as_bytes())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            b"tileset_id" => Ok(Field::TilesetId),
            b"width" => Ok(Field::Width),
            b"height" => Ok(Field::Height),
            b"autoplay_bgm" => Ok(Field::AutoplayBgm),
            b"bgm" => Ok(Field::Bgm),
            b"autoplay_bgs" => Ok(Field::AutoplayBgs),
            b"bgs" => Ok(Field::Bgs),
            b"encounter_list" => Ok(Field::EncounterList),
            b"encounter_step" => Ok(Field::EncounterStep),
            b"data" => Ok(Field::Data),
            b"events" => Ok(Field::Events),
            _ => Ok(Field::Ignore),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'res, 'de> serde::de::Visitor<'de> for Visitor<'res> {
    type Value = Map;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("struct Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
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

        while let Some(field) = map.next_key()? {
            match field {
                Field::TilesetId => {
                    let id = map.next_value::<id_serde::DeserializeId>()?;
                    tileset_id = Some(id.0);
                }
                Field::Width => width = Some(map.next_value()?),
                Field::Height => height = Some(map.next_value()?),
                Field::AutoplayBgm => autoplay_bgm = Some(map.next_value()?),
                Field::Bgm => bgm = Some(map.next_value()?),
                Field::AutoplayBgs => autoplay_bgs = Some(map.next_value()?),
                Field::Bgs => bgs = Some(map.next_value()?),
                Field::EncounterList => encounter_list = Some(map.next_value()?),
                Field::EncounterStep => encounter_step = Some(map.next_value()?),
                Field::Data => data = Some(map.next_value()?),
                Field::Events => {
                    todo!();
                }
                Field::Ignore => {}
            }
        }

        Ok(Map {
            tileset_id: tileset_id.ok_or_else(|| A::Error::missing_field("tileset_id"))?,
            width: width.ok_or_else(|| A::Error::missing_field("width"))?,
            height: height.ok_or_else(|| A::Error::missing_field("height"))?,
            autoplay_bgm: autoplay_bgm.ok_or_else(|| A::Error::missing_field("autoplay_bgm"))?,
            bgm: bgm.ok_or_else(|| A::Error::missing_field("bgm"))?,
            autoplay_bgs: autoplay_bgs.ok_or_else(|| A::Error::missing_field("autoplay_bgs"))?,
            bgs: bgs.ok_or_else(|| A::Error::missing_field("bgs"))?,
            encounter_list: encounter_list
                .ok_or_else(|| A::Error::missing_field("encounter_list"))?,
            encounter_step: encounter_step
                .ok_or_else(|| A::Error::missing_field("encounter_step"))?,
            data: data.ok_or_else(|| A::Error::missing_field("data"))?,
            events: events.ok_or_else(|| A::Error::missing_field("events"))?,
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
        let MapDeserializer { command_db } = self;
        deserializer.deserialize(Visitor { command_db })
    }
}

impl<'res, 'de> serde::de::DeserializeSeed<'de> for MapDeserializer<'res> {
    type Value = Map;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = &[
            "tileset_id",
            "width",
            "height",
            "autoplay_bgm",
            "bgm",
            "autoplay_bgs",
            "bgs",
            "encounter_list",
            "encounter_step",
            "data",
            "events",
        ];
        let MapDeserializer { command_db } = self;
        deserializer.deserialize_struct("Map", fields, Visitor { command_db })
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
        let MapSerializer { command_db, map } = *self;
        let mut ivars = serializer.serialize_object("RPG::Map".into(), 11)?;

        ivars.serialize_entry("@tileset_id".into(), &id_alox::SerializeId(map.tileset_id))?;
        ivars.serialize_entry("@width".into(), &map.width)?;
        ivars.serialize_entry("@height".into(), &map.height)?;
        ivars.serialize_entry("@autoplay_bgm".into(), &map.autoplay_bgm)?;
        ivars.serialize_entry("@bgm".into(), &map.bgm)?;
        ivars.serialize_entry("@autoplay_bgs".into(), &map.autoplay_bgs)?;
        ivars.serialize_entry("@bgs".into(), &map.bgs)?;
        ivars.serialize_entry("@encounter_list".into(), &map.encounter_list)?;
        ivars.serialize_entry("@encounter_step".into(), &map.encounter_step)?;
        ivars.serialize_entry("@data".into(), &map.data)?;
        ivars.serialize_entry("@events".into(), &map.events)?;

        ivars.end()
    }
}

impl<'res> serde::Serialize for MapSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let MapSerializer { command_db, map } = *self;
        let mut fields = serializer.serialize_struct("Map", 11)?;

        fields.serialize_field("tileset_id", &id_serde::SerializeId(map.tileset_id))?;
        fields.serialize_field("width", &map.width)?;
        fields.serialize_field("height", &map.height)?;
        fields.serialize_field("autoplay_bgm", &map.autoplay_bgm)?;
        fields.serialize_field("bgm", &map.bgm)?;
        fields.serialize_field("autoplay_bgs", &map.autoplay_bgs)?;
        fields.serialize_field("bgs", &map.bgs)?;
        fields.serialize_field("encounter_list", &map.encounter_list)?;
        fields.serialize_field("encounter_step", &map.encounter_step)?;
        fields.serialize_field("data", &map.data)?;
        fields.serialize_field("events", &map.events)?;

        fields.end()
    }
}
