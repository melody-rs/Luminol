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
use crate::rpg::{AudioFile, Event, RawEvent};
use crate::{id_alox, id_serde, option_vec, CommandDB, Table3};

#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Default, Debug)]
#[marshal(class = "RPG::Map")]
#[serde(rename = "Map")]
pub struct RawMap {
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
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
    pub events: option_vec::OptionVec<RawEvent>,
}

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

pub struct MapSerializer<'res> {
    command_db: &'res CommandDB,
    map: &'res Map,
}

impl<'res> MapSerializer<'res> {
    pub fn new(command_db: &'res CommandDB, map: &'res Map) -> Self {
        Self { command_db, map }
    }
}

impl RawMap {
    pub fn parse_commands(self, command_db: &CommandDB) -> Map {
        let events = self
            .events
            .into_iter()
            .map(|(index, event)| (index, event.parse_commands(command_db)))
            .collect();
        Map {
            tileset_id: self.tileset_id,
            width: self.width,
            height: self.height,
            autoplay_bgm: self.autoplay_bgm,
            bgm: self.bgm,
            autoplay_bgs: self.autoplay_bgs,
            bgs: self.bgs,
            encounter_list: self.encounter_list,
            encounter_step: self.encounter_step,
            data: self.data,
            events,
            modified: false,
        }
    }
}

impl alox_48::Serialize for MapSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        todo!()
    }
}

impl serde::Serialize for MapSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
