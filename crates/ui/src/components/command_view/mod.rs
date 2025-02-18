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
//
//     Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining
// it with Steamworks API by Valve Corporation, containing parts covered by
// terms of the Steamworks API by Valve Corporation, the licensors of this
// Program grant you additional permission to convey the resulting work.

use indextree::{Arena, NodeId};
use luminol_data::commands::{Command as CommandDesc, CommandKind};
use luminol_data::{rpg, CommandDB};

pub struct CommandView {
    id_source: egui::Id,
    state: State,
}

enum State {
    Viewing,
    Insert { under: NodeId },
    Editing { which: NodeId },
}

impl CommandView {
    pub fn new(id_source: egui::Id) -> Self {
        Self {
            id_source,
            state: State::Viewing,
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Viewing;
    }

    fn display_insert(&mut self, ui: &mut egui::Ui, under: NodeId) {
        if ui.button(">").clicked() {
            self.state = State::Insert { under }
        }
    }

    fn collapsing_state_for(
        &mut self,
        ctx: &egui::Context,
        node_id: NodeId,
    ) -> egui::collapsing_header::CollapsingState {
        let id = egui::Id::new("cmd_edit_branch")
            .with(self.id_source)
            .with(node_id);
        egui::collapsing_header::CollapsingState::load_with_default_open(ctx, id, true)
    }

    fn ui_for_command(
        &mut self,
        ui: &mut egui::Ui,
        command_db: &CommandDB,
        commands: &Arena<rpg::EventCommand>,
        this_id: NodeId,
    ) {
        let command = commands[this_id].get();
        match command_db.get(command.code) {
            Some(desc) => match &desc.kind {
                CommandKind::Branch {
                    branches,
                    terminator,
                    command_contains_branch,
                    ..
                } => {
                    let mut children = this_id.children(commands);
                    self.collapsing_state_for(ui.ctx(), this_id)
                        .show_header(ui, |ui| {
                            let resp = ui.button(egui::RichText::new(&desc.name).color(desc.color));
                            if resp.clicked() {
                                self.state = State::Editing { which: this_id }
                            }
                        })
                        .body(|ui| {
                            if *command_contains_branch {
                                let branch = children.next().unwrap();
                                for child in branch.children(commands) {
                                    self.ui_for_command(ui, command_db, commands, child);
                                }
                                self.display_insert(ui, this_id);
                            }
                        });
                    for branch in children {
                        let code = commands[branch].get().code;
                        let Some(branch_desc) = branches.iter().find(|b| b.code == code) else {
                            continue;
                        };
                        self.collapsing_state_for(ui.ctx(), branch)
                            .show_header(ui, |ui| {
                                ui.label(egui::RichText::new(&branch_desc.name).color(desc.color));
                            })
                            .body(|ui| {
                                for child in branch.children(commands) {
                                    self.ui_for_command(ui, command_db, commands, child);
                                }
                                self.display_insert(ui, this_id);
                            });
                    }
                    ui.indent("??", |ui| {
                        ui.label(egui::RichText::new(&terminator.name).color(desc.color));
                    });
                }
                CommandKind::Multi(_) => {
                    ui.label(egui::RichText::new(&desc.name).color(desc.color));
                    let text = command.parameters[0].as_string().unwrap();
                    ui.indent("??", |ui| ui.label(text));
                }
                CommandKind::Regular { .. } => {
                    let resp = ui.button(egui::RichText::new(&desc.name).color(desc.color));
                    if resp.clicked() {
                        self.state = State::Editing { which: this_id }
                    }
                }
                CommandKind::MoveRoute(_) => {
                    ui.label(egui::RichText::new(&desc.name).color(desc.color));
                    let route = command.parameters[0].as_moveroute().unwrap();
                    ui.indent("??", |ui| {
                        for command in route.list.iter() {
                            // TODO
                            let text =
                                egui::RichText::new(format!("{command:?}")).color(desc.color);
                            ui.label(text);
                        }
                    });
                }
                CommandKind::Blank => {
                    ui.label(egui::RichText::new(&desc.name).color(desc.color));
                }
            },
            None => {
                let text = egui::RichText::new(format!("🔥 Unrecognized command {}", command.code))
                    .color(egui::Color32::RED);
                ui.label(text);
            }
        }
    }

    fn default_for_command(command: &CommandDesc) -> rpg::EventCommand {
        todo!()
    }

    // maybe take a Ctx parameter instead? we have to keep passing all these parameters around which gets annoying fast
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        update_state: &mut luminol_core::UpdateState<'_>,
        commands: &mut Arena<rpg::EventCommand>,
        root_node: NodeId,
    ) {
        let project_config = update_state.project_config.as_ref().unwrap();
        let command_db = &project_config.command_db;
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for child in root_node.children(commands) {
                    self.ui_for_command(ui, command_db, commands, child);
                }
                self.display_insert(ui, root_node);
            });
        match self.state {
            State::Viewing => {}
            State::Insert { under } => {
                let mut is_open = true;
                let id = egui::Id::new("cmd_edit_insert")
                    .with(self.id_source)
                    .with(under);
                egui::Window::new("Insert Command")
                    .id(id)
                    .open(&mut is_open)
                    .show(ui.ctx(), |ui| {
                        for (id, command) in command_db.iter() {
                            if ui.button(&command.name).clicked() {
                                let default = Self::default_for_command(command);
                                let node_id = under.append_value(default, commands);
                                self.state = State::Editing { which: node_id }
                            }
                        }
                    });
                if !is_open {
                    self.state = State::Viewing;
                }
            }
            State::Editing { which } => {
                let mut is_open = true;
                let id = egui::Id::new("cmd_edit_edit")
                    .with(self.id_source)
                    .with(which);
                egui::Window::new("Edit Command")
                    .id(id)
                    .open(&mut is_open)
                    .show(ui.ctx(), |ui| {});
                if !is_open {
                    self.state = State::Viewing;
                }
            }
        }
    }
}
