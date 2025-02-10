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
use crate::{
    id_alox, id_serde, optional_id_alox, optional_id_serde, optional_path_alox,
    optional_path_serde, rpg::MoveRoute, BlendMode, ParameterType, Path,
};
use alox_48::{DeError, SerializeIvars};

#[derive(Debug, Clone)]
pub struct Event {
    pub id: usize,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub pages: Vec<EventPage>,

    pub extra_data: EventExtraData,
}

#[derive(Debug, Default, Clone)]
pub struct EventExtraData {
    /// Whether or not the event editor for this event is open
    pub is_editor_open: bool,
    pub graphic_modified: std::cell::Cell<bool>,
}

impl Event {
    #[must_use]
    pub fn new(x: i32, y: i32, id: usize) -> Self {
        Self {
            id,
            name: format!("EV{id:0>3}"),
            x,
            y,
            pages: vec![EventPage::default()],

            extra_data: EventExtraData::default(),
        }
    }
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::CommonEvent")]
pub struct CommonEvent {
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub id: usize,
    pub name: String,
    pub trigger: usize,
    pub switch_id: usize,
    pub list: Vec<EventCommand>,
}

#[derive(Debug, Clone)]
pub struct EventPage {
    pub condition: EventCondition,
    pub graphic: Graphic,
    pub move_type: MoveType,
    pub move_speed: MoveSpeed,
    pub move_frequency: MoveFreq,
    pub move_route: MoveRoute,
    pub walk_anime: bool,
    pub step_anime: bool,
    pub direction_fix: bool,
    pub through: bool,
    pub always_on_top: bool,
    pub trigger: EventTrigger,
    pub list: Vec<EventCommand>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum EventTrigger {
    #[strum(to_string = "Action Button")]
    ActionButton,
    #[strum(to_string = "Player Touch")]
    PlayerTouch,
    #[strum(to_string = "Event Touch")]
    EventTouch,
    #[strum(to_string = "Autorun")]
    Autorun,
    #[strum(to_string = "Parallel Process")]
    Parallel,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveType {
    Fixed,
    Random,
    Approach,
    Custom,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveFreq {
    Lowest = 1,
    Lower,
    Low,
    High,
    Higher,
    Highest,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveSpeed {
    Slowest = 1,
    Slower,
    Slow,
    Fast,
    Faster,
    Fastest,
}

impl Default for EventPage {
    fn default() -> Self {
        Self {
            condition: EventCondition::default(),
            graphic: Graphic::default(),
            move_type: MoveType::Fixed,
            move_speed: MoveSpeed::Slow,
            move_frequency: MoveFreq::Low,
            move_route: MoveRoute::default(),
            walk_anime: true,
            step_anime: false,
            direction_fix: false,
            through: false,
            always_on_top: false,
            trigger: EventTrigger::ActionButton,
            list: vec![],
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page::Graphic")]
pub struct Graphic {
    #[serde(with = "optional_id_serde")]
    #[marshal(with = "optional_id_alox")]
    pub tile_id: Option<usize>,
    #[serde(with = "optional_path_serde")]
    #[marshal(with = "optional_path_alox")]
    pub character_name: Path,
    pub character_hue: i32,
    pub direction: i32,
    pub pattern: i32,
    pub opacity: i32,
    pub blend_type: BlendMode,
}

impl Default for Graphic {
    fn default() -> Self {
        Self {
            tile_id: None,
            character_name: None,
            character_hue: 0,
            direction: 2,
            pattern: 0,
            opacity: 255,
            blend_type: BlendMode::Normal,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page::Condition")]
pub struct EventCondition {
    pub switch1_valid: bool,
    pub switch2_valid: bool,
    pub variable_valid: bool,
    pub self_switch_valid: bool,
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub switch1_id: usize,
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub switch2_id: usize,
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub variable_id: usize,
    pub variable_value: i32,
    pub self_switch_ch: SelfSwitch,
}

impl Default for EventCondition {
    fn default() -> Self {
        Self {
            switch1_valid: false,
            switch2_valid: false,
            variable_valid: false,
            self_switch_valid: false,
            switch1_id: 0,
            switch2_id: 0,
            variable_id: 0,
            variable_value: 0,
            self_switch_ch: SelfSwitch::A,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(from = "String", into = "String")]
#[marshal(from = "String", into = "String")]
pub enum SelfSwitch {
    A,
    B,
    C,
    D,
}

impl From<String> for SelfSwitch {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            _ => panic!("wrong value for self switch"),
        }
    }
}

impl From<SelfSwitch> for String {
    fn from(val: SelfSwitch) -> Self {
        match val {
            SelfSwitch::A => "A".to_string(),
            SelfSwitch::B => "B".to_string(),
            SelfSwitch::C => "C".to_string(),
            SelfSwitch::D => "D".to_string(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[allow(missing_docs)]
#[marshal(class = "RPG::EventCommand")]
pub struct EventCommand {
    pub code: u16,
    pub indent: usize,
    pub parameters: Vec<ParameterType>,
}

pub struct EventDeserializer<'res> {
    pub command_db: &'res crate::CommandDB,
}

struct EventVisitor<'res> {
    command_db: &'res crate::CommandDB,
}

impl<'res, 'de> alox_48::de::DeserializeSeed<'de> for EventDeserializer<'res> {
    type Value = Event;

    fn deserialize<D>(self, deserializer: D) -> alox_48::DeResult<Self::Value>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        let Self { command_db } = self;
        deserializer.deserialize(EventVisitor { command_db })
    }
}

impl<'res, 'de> alox_48::de::Visitor<'de> for EventVisitor<'res> {
    type Value = Event;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an instance of RPG::Event")
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
        let mut id = None;
        let mut name = None;
        let mut x = None;
        let mut y = None;
        let mut pages = None;

        while let Some(field) = instance_variables.next_ivar()? {
            match field.to_rust_field_name().unwrap_or(field).as_str() {
                "id" => id = Some(instance_variables.next_value()?),
                "name" => name = Some(instance_variables.next_value()?),
                "x" => x = Some(instance_variables.next_value()?),
                "y" => y = Some(instance_variables.next_value()?),
                "pages" => {
                    let seed = MultiPageDeserializer { command_db };
                    let value = instance_variables.next_value_seed(seed)?;
                    pages = Some(value);
                }
                _ => {
                    instance_variables.next_value::<alox_48::de::Ignored>()?;
                }
            }
        }

        Ok(Event {
            id: id.ok_or_else(|| DeError::missing_field("id".into()))?,
            name: name.ok_or_else(|| DeError::missing_field("name".into()))?,
            x: x.ok_or_else(|| DeError::missing_field("x".into()))?,
            y: y.ok_or_else(|| DeError::missing_field("y".into()))?,
            pages: pages.ok_or_else(|| DeError::missing_field("pages".into()))?,
            extra_data: EventExtraData::default(),
        })
    }
}

struct MultiPageDeserializer<'res> {
    command_db: &'res crate::CommandDB,
}

impl<'res, 'de> alox_48::de::DeserializeSeed<'de> for MultiPageDeserializer<'res> {
    type Value = Vec<EventPage>;

    fn deserialize<D>(self, deserializer: D) -> alox_48::DeResult<Self::Value>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        struct Visitor<'res> {
            command_db: &'res crate::CommandDB,
        }
        impl<'res, 'de> alox_48::de::Visitor<'de> for Visitor<'res> {
            type Value = Vec<EventPage>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array of RPG::Event::Page")
            }

            fn visit_array<A>(self, mut array: A) -> alox_48::DeResult<Self::Value>
            where
                A: alox_48::ArrayAccess<'de>,
            {
                let Self { command_db } = self;
                let mut data = Vec::with_capacity(array.len());
                while let Some(page) =
                    array.next_element_seed(EventPageDeserializer { command_db })?
                {
                    data.push(page);
                }
                Ok(data)
            }
        }

        let Self { command_db } = self;
        deserializer.deserialize(Visitor { command_db })
    }
}

struct EventPageDeserializer<'res> {
    command_db: &'res crate::CommandDB,
}

struct EventPageVisitor<'res> {
    command_db: &'res crate::CommandDB,
}

impl<'res, 'de> alox_48::de::Visitor<'de> for EventPageVisitor<'res> {
    type Value = EventPage;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an instance of RPG::Event::Page")
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
        let mut condition = None;
        let mut graphic = None;
        let mut move_type = None;
        let mut move_speed = None;
        let mut move_frequency = None;
        let mut move_route = None;
        let mut walk_anime = None;
        let mut step_anime = None;
        let mut direction_fix = None;
        let mut through = None;
        let mut always_on_top = None;
        let mut trigger = None;
        let mut list = None;

        while let Some(field) = instance_variables.next_ivar()? {
            match field.to_rust_field_name().unwrap_or(field).as_str() {
                "condition" => condition = Some(instance_variables.next_value()?),
                "graphic" => graphic = Some(instance_variables.next_value()?),
                "move_type" => move_type = Some(instance_variables.next_value()?),
                "move_speed" => move_speed = Some(instance_variables.next_value()?),
                "move_frequency" => move_frequency = Some(instance_variables.next_value()?),
                "move_route" => move_route = Some(instance_variables.next_value()?),
                "walk_anime" => walk_anime = Some(instance_variables.next_value()?),
                "step_anime" => step_anime = Some(instance_variables.next_value()?),
                "direction_fix" => direction_fix = Some(instance_variables.next_value()?),
                "through" => through = Some(instance_variables.next_value()?),
                "always_on_top" => always_on_top = Some(instance_variables.next_value()?),
                "trigger" => trigger = Some(instance_variables.next_value()?),
                "list" => {
                    // TODO
                    list = Some(instance_variables.next_value()?);
                }
                _ => {
                    instance_variables.next_value::<alox_48::de::Ignored>()?;
                }
            }
        }

        Ok(EventPage {
            condition: condition.ok_or_else(|| DeError::missing_field("condition".into()))?,
            graphic: graphic.ok_or_else(|| DeError::missing_field("graphic".into()))?,
            move_type: move_type.ok_or_else(|| DeError::missing_field("move_type".into()))?,
            move_speed: move_speed.ok_or_else(|| DeError::missing_field("move_speed".into()))?,
            move_frequency: move_frequency
                .ok_or_else(|| DeError::missing_field("move_frequency".into()))?,
            move_route: move_route.ok_or_else(|| DeError::missing_field("move_route".into()))?,
            walk_anime: walk_anime.ok_or_else(|| DeError::missing_field("walk_anime".into()))?,
            step_anime: step_anime.ok_or_else(|| DeError::missing_field("step_anime".into()))?,
            direction_fix: direction_fix
                .ok_or_else(|| DeError::missing_field("direction_fix".into()))?,
            through: through.ok_or_else(|| DeError::missing_field("through".into()))?,
            always_on_top: always_on_top
                .ok_or_else(|| DeError::missing_field("always_on_top".into()))?,
            trigger: trigger.ok_or_else(|| DeError::missing_field("trigger".into()))?,
            list: list.ok_or_else(|| DeError::missing_field("list".into()))?,
        })
    }
}

impl<'res, 'de> alox_48::de::DeserializeSeed<'de> for EventPageDeserializer<'res> {
    type Value = EventPage;

    fn deserialize<D>(self, deserializer: D) -> alox_48::DeResult<Self::Value>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        let Self { command_db } = self;
        deserializer.deserialize(EventPageVisitor { command_db })
    }
}

pub struct EventSerializer<'res> {
    pub command_db: &'res crate::CommandDB,
    pub event: &'res Event,
}

impl<'res> alox_48::Serialize for EventSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let Self { command_db, event } = *self;
        let Event {
            id,
            name,
            x,
            y,
            pages,
            ..
        } = event;
        let mut ivars = serializer.serialize_object("RPG::Event".into(), 5)?;

        ivars.serialize_entry("@id".into(), id)?;
        ivars.serialize_entry("@name".into(), name)?;
        ivars.serialize_entry("@x".into(), x)?;
        ivars.serialize_entry("@y".into(), y)?;
        ivars.serialize_entry("@pages".into(), &MultiPageSerializer { command_db, pages })?;

        ivars.end()
    }
}

struct MultiPageSerializer<'res> {
    pub command_db: &'res crate::CommandDB,
    pub pages: &'res [EventPage],
}

impl<'res> alox_48::Serialize for MultiPageSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let Self { command_db, pages } = *self;
        let iter = pages.iter().map(|page| PageSerializer { page, command_db });
        serializer.collect_array(iter)
    }
}

struct PageSerializer<'res> {
    pub command_db: &'res crate::CommandDB,
    pub page: &'res EventPage,
}

impl<'res> alox_48::Serialize for PageSerializer<'res> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let Self { command_db, page } = *self;
        let EventPage {
            condition,
            graphic,
            move_type,
            move_speed,
            move_frequency,
            move_route,
            walk_anime,
            step_anime,
            direction_fix,
            through,
            always_on_top,
            trigger,
            list,
        } = page;
        let mut ivars = serializer.serialize_object("RPG::Event::Page".into(), 13)?;

        ivars.serialize_entry("@condition".into(), condition)?;
        ivars.serialize_entry("@graphic".into(), graphic)?;
        ivars.serialize_entry("@move_type".into(), move_type)?;
        ivars.serialize_entry("@move_speed".into(), move_speed)?;
        ivars.serialize_entry("@move_frequency".into(), move_frequency)?;
        ivars.serialize_entry("@move_route".into(), move_route)?;
        ivars.serialize_entry("@walk_anime".into(), walk_anime)?;
        ivars.serialize_entry("@step_anime".into(), step_anime)?;
        ivars.serialize_entry("@direction_fix".into(), direction_fix)?;
        ivars.serialize_entry("@through".into(), through)?;
        ivars.serialize_entry("@always_on_top".into(), always_on_top)?;
        ivars.serialize_entry("@trigger".into(), trigger)?;
        ivars.serialize_entry("@list".into(), list)?;

        ivars.end()
    }
}
