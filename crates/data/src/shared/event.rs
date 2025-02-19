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
    commands::CommandKind, id_alox, id_serde, optional_id_alox, optional_id_serde,
    optional_path_alox, optional_path_serde, rpg::MoveRoute, BlendMode, CommandDB, ParameterType,
    Path,
};
use std::fmt::Write;
use std::iter::Peekable;

#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, Clone)]
#[marshal(class = "RPG::Event")]
#[serde(rename = "Event")]
pub struct RawEvent {
    pub id: usize,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub pages: Vec<RawEventPage>,
}

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

impl RawEvent {
    pub fn parse_commands(self, command_db: &CommandDB) -> Event {
        let pages = self
            .pages
            .into_iter()
            .map(|p| p.parse_commands(command_db))
            .collect();
        Event {
            id: self.id,
            name: self.name,
            x: self.x,
            y: self.y,
            pages,
            extra_data: EventExtraData::default(),
        }
    }
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

#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, Clone)]
#[marshal(class = "RPG::Event::Page")]
#[serde(rename = "EventPage")]
pub struct RawEventPage {
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

impl RawEventPage {
    fn parse_command_under(
        parent: indextree::NodeId,
        command: EventCommand,
        commands: &mut indextree::Arena<EventCommand>,
        iter: &mut Peekable<impl Iterator<Item = EventCommand>>,
        command_db: &CommandDB,
    ) {
        if command.code == 0 {
            return;
        }

        let desc = command_db.get(command.code);
        let current = parent.append_value(command, commands);
        if let Some(desc) = desc {
            // TODO validate parameters?
            match &desc.kind {
                CommandKind::Branch {
                    branches,
                    terminator,
                    command_contains_branch,
                    ..
                } => {
                    let mut branch = None;
                    if *command_contains_branch {
                        branch = Some(current.append_value(EventPage::ROOT_NODE, commands))
                    }
                    loop {
                        let next = iter.next().unwrap();

                        if next.code == terminator.code {
                            break;
                        }

                        if branches.iter().any(|branch| branch.code == next.code) {
                            branch = Some(current.append_value(next, commands));
                            continue;
                        }

                        if let Some(branch) = branch {
                            Self::parse_command_under(branch, next, commands, iter, command_db);
                        }
                    }
                }
                CommandKind::Multi { cont, .. } => {
                    let command = commands[current].get_mut();
                    let text = command.parameters[0].as_string_mut().unwrap();
                    while let Some(next) = iter.next_if(|next| next.code == *cont) {
                        let next_line = next.parameters[0].as_string().unwrap();
                        write!(text, "\n{next_line}").unwrap();
                    }
                }
                CommandKind::MoveRoute(display_id) => {
                    // consume all editor display id move routes
                    while iter.next_if(|next| next.code == *display_id).is_some() {}
                }
                // we've already inserted the command- nothing else to do.
                CommandKind::Regular { .. } | CommandKind::Blank => {}
            }
        }
    }

    pub fn parse_commands(self, command_db: &CommandDB) -> EventPage {
        let mut commands = indextree::Arena::with_capacity(self.list.len() + 1); // +1 because we need to store the root node
        let root = commands.new_node(EventPage::ROOT_NODE);
        let mut iter = self.list.into_iter().peekable();
        while let Some(command) = iter.next() {
            Self::parse_command_under(root, command, &mut commands, &mut iter, command_db);
        }
        let printable = root.debug_pretty_print(&commands);
        println!("{printable:?}");

        EventPage {
            condition: self.condition,
            graphic: self.graphic,
            move_type: self.move_type,
            move_speed: self.move_speed,
            move_frequency: self.move_frequency,
            move_route: self.move_route,
            walk_anime: self.walk_anime,
            step_anime: self.step_anime,
            direction_fix: self.direction_fix,
            through: self.through,
            always_on_top: self.always_on_top,
            trigger: self.trigger,

            commands,
            root,
        }
    }
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

    pub commands: indextree::Arena<EventCommand>,
    // this is probably always going to be the same node id, can we avoid storing it somehow?
    pub root: indextree::NodeId,
}

impl EventPage {
    pub const ROOT_NODE: EventCommand = EventCommand {
        code: u16::MAX,
        indent: 0,
        parameters: vec![],
    };
}

impl Default for EventPage {
    fn default() -> Self {
        let mut commands = indextree::Arena::new();
        let root = commands.new_node(Self::ROOT_NODE);
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

            commands,
            root,
        }
    }
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
