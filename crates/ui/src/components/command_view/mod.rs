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

use luminol_core::UpdateState;
use luminol_data::{commands::CommandKind, rpg};

#[derive(Default)]
pub enum CommandView {
    #[default]
    Viewing,
}

impl CommandView {
    pub fn new() -> Self {
        CommandView::default()
    }

    pub fn reset(&mut self) {
        *self = CommandView::Viewing;
    }

    fn display_insert(&mut self, ui: &mut egui::Ui) {
        ui.label(">");
    }

    fn ui_for_command(
        &mut self,
        ui: &mut egui::Ui,
        command_db: &luminol_data::CommandDB,
        commands: &indextree::Arena<rpg::EventCommand>,
        node_id: indextree::NodeId,
    ) {
        let command = commands[node_id].get();
        match command_db.get(command.code) {
            Some(desc) => match &desc.kind {
                CommandKind::Branch {
                    parameters,
                    branches,
                    terminator,
                    command_contains_branch,
                } => {
                    let id = egui::Id::new("event_edit_branch").with(node_id);
                    let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        id,
                        true,
                    );
                    let mut children = node_id.children(commands);
                    header
                        .show_header(ui, |ui| {
                            ui.label(egui::RichText::new(&desc.name).color(desc.color));
                        })
                        .body(|ui| {
                            if *command_contains_branch {
                                let branch = children.next().unwrap();
                                for child in branch.children(commands) {
                                    self.ui_for_command(ui, command_db, commands, child);
                                }
                                self.display_insert(ui);
                            }
                        });
                    for branch in children {
                        let code = commands[branch].get().code;
                        let Some(branch_desc) = branches.iter().find(|b| b.code == code) else {
                            continue;
                        };
                        let id = egui::Id::new("event_edit_branch").with(branch);
                        let header =
                            egui::collapsing_header::CollapsingState::load_with_default_open(
                                ui.ctx(),
                                id,
                                true,
                            );
                        header
                            .show_header(ui, |ui| {
                                ui.label(egui::RichText::new(&branch_desc.name).color(desc.color));
                            })
                            .body(|ui| {
                                for child in branch.children(commands) {
                                    self.ui_for_command(ui, command_db, commands, child);
                                }
                                self.display_insert(ui);
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
                CommandKind::Regular { parameters } => {
                    ui.label(egui::RichText::new(&desc.name).color(desc.color));
                }
                CommandKind::MoveRoute(_) => todo!(),
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

    // maybe take a Ctx parameter instead? we have to keep passing all these parameters around which gets annoying fast
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        update_state: &mut UpdateState<'_>,
        commands: &mut indextree::Arena<rpg::EventCommand>,
        root_node: indextree::NodeId,
    ) {
        let project_config = update_state.project_config.as_ref().unwrap();
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for child in root_node.children(commands) {
                    self.ui_for_command(ui, &project_config.command_db, commands, child);
                }
                self.display_insert(ui);
            });
    }
}
