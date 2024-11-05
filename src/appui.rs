use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Gauge, HighlightSpacing, List, ListItem, StatefulWidget, Widget,
    },
};

use crate::app::Musicfile;
use crate::app::{App, PlayingItem, StatusOfMusicFile, StatusOfPlayingItem};

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::Rgb(143, 188, 187))
    .fg(Color::Rgb(216, 222, 233))
    .add_modifier(Modifier::BOLD);

const TODO_COLRO: ratatui::prelude::Color = Color::Rgb(143, 188, 187);

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, main_area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]).areas(main_area);
        let [top_right_area, bottom_right_area] =
            Layout::vertical([Constraint::Fill(3), Constraint::Length(5)]).areas(right_area);

        match self.apptab {
            crate::app::AppTab::Music => {
                self.render_music_list(left_area, buf);
                self.render_playing_list(top_right_area, buf);
                self.draw_playing_music(bottom_right_area, buf);
            }
            crate::app::AppTab::Helper => self.helper(main_area, buf),
        }
    }
}

impl App {
    fn render_music_list(&mut self, area: Rect, buf: &mut Buffer) {
        let music_list_display =
            &mut self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display];

        let number_of_music_file = music_list_display.items.len();
        let title = format!("Music List | {} songs", number_of_music_file);
        let block = Block::new()
            .title(Line::raw(title).centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::Rgb(143, 188, 187));

        let items: Vec<ListItem> = music_list_display
            .items
            .iter()
            .enumerate()
            .map(|(_, todo_item)| ListItem::from(todo_item))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut music_list_display.state);
    }

    fn render_playing_list(&mut self, area: Rect, buf: &mut Buffer) {
        let number_of_playing_music = self.playing_list.items.len();
        let total_time = display_time(self.playing_list.total_time);
        let title = format!(
            "Playing List | {} Songs | Total Time {} ",
            number_of_playing_music, total_time
        );
        let block = Block::new()
            .title(Line::raw(title).centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::Rgb(143, 188, 187));

        if self.is_stop() {
            self.handle_stop_music();
        }

        let items: Vec<ListItem> = self
            .playing_list
            .items
            .iter()
            .enumerate()
            .map(|(_i, todo_item)| ListItem::from(todo_item))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.playing_list.state);

        // let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    }

    fn draw_playing_music(&mut self, area: Rect, buf: &mut Buffer) {
        let mut block_title: Vec<Span> =
            vec![Span::styled(" Playing ", Style::default().fg(TODO_COLRO))];

        let mut gauge_title: Vec<Span> = Vec::new();

        let label = if self.musichandle.is_paused() {
            "  "
        } else if !self.is_stop() && self.musichandle.is_empty() {
            ""
        } else if self.is_stop() {
            " 󰓛 "
        } else {
            "  "
        };

        let play_style_icon = match self.playing_list.playingmod {
            crate::app::PlayingMod::Auto => "",
            crate::app::PlayingMod::Repeat => "",
            crate::app::PlayingMod::Random => "",
            crate::app::PlayingMod::Manual => "",
        };

        let playing_music_index = self.playing_list.playing_music_index;
        let playing_music_name = if playing_music_index != -1 {
            self.playing_list.items[playing_music_index as usize]
                .path_of_music
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        } else {
            "".to_string()
        };
        block_title.push(Span::styled(" ", Style::default().fg(TODO_COLRO)));
        block_title.push(Span::styled(
            format!("{} ", playing_music_name),
            Style::default().fg(TODO_COLRO).add_modifier(Modifier::BOLD),
        ));

        let play_dur = self.musichandle.time_played();

        let total_dur = if playing_music_index == -1 {
            0
        } else {
            self.playing_list.items[playing_music_index as usize].length
        };
        gauge_title.push(Span::styled(
            format!(
                " [ {}m {}s : {}m {}s ] {} ",
                play_dur / 60,
                play_dur % 60,
                total_dur / 60,
                total_dur % 60,
                play_style_icon,
            ),
            Style::default().fg(TODO_COLRO),
        ));
        let volume = self.musichandle.get_volume();
        block_title.push(Span::styled(
            match volume {
                v if v >= 0.7 => " ",
                v if v >= 0.3 => "",
                v if v > 0.0 => "",
                _ => "",
            },
            Style::default().fg(TODO_COLRO),
        ));
        let volume = if volume > 0.0 { volume } else { 0.0 };
        block_title.push(Span::styled(
            format!("{:3.0}% ", volume * 100.0),
            Style::default().fg(TODO_COLRO),
        ));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(TODO_COLRO))
            .title(block_title)
            .title_alignment(Alignment::Center);
        block.render(area, buf);

        let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(TODO_COLRO))
                    .title(gauge_title),
            )
            .gauge_style(Style::default().fg(TODO_COLRO))
            .label(Span::styled(label, Style::default().fg(TODO_COLRO)))
            .ratio(self.song_progress());
        gauge.render(inner_rect, buf)
    }

    fn helper(&mut self, area: Rect, buf: &mut Buffer) {
        let help_table = &mut self.control_table;
        let rows = help_table.items.iter().map(|item| {
            let cells = item.iter().map(|c| ratatui::widgets::Cell::from(c.clone()));
            ratatui::widgets::Row::new(cells)
                .bottom_margin(1)
        });

        let widths = [Constraint::Length(2), Constraint::Length(2)];
        let table = ratatui::widgets::Table::new(rows, widths)
            .block(Block::default().borders(Borders::ALL).title("Helper"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(60),
                Constraint::Min(10),
            ]);

        StatefulWidget::render(table, area, buf, &mut help_table.state);
    }
}

impl From<&Musicfile> for ListItem<'_> {
    fn from(value: &Musicfile) -> Self {
        let path_str = value
            .info
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        if value.info.is_file() {
            let line = match value.status {
                StatusOfMusicFile::Added => {
                    let pre = if value.num_added > 1 {
                        value.num_added.to_string()
                    } else {
                        "".to_string()
                    };
                    Line::styled(format!("{}+{}", pre, path_str), Color::Rgb(143, 188, 187))
                }
                StatusOfMusicFile::NotAdded => {
                    Line::styled(format!(" {}", path_str), Color::Rgb(216, 222, 233))
                }
            };
            ListItem::new(line)
        } else if value.info.is_dir() {
            let line = Line::styled(format!(" {}", path_str), Color::Rgb(216, 222, 233));
            ListItem::new(line)
        } else {
            // todo!();
            let line = Line::styled("", Color::Rgb(143, 188, 187));
            ListItem::new(line)
        }
    }
}

impl From<&PlayingItem> for ListItem<'_> {
    fn from(value: &PlayingItem) -> Self {
        let path_str = value
            .path_of_music
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let line = match value.status {
            StatusOfPlayingItem::Playing => {
                Line::styled(format!(" {}", path_str), Color::Rgb(143, 188, 187))
            }
            StatusOfPlayingItem::Pause => {
                Line::styled(format!(" {}", path_str), Color::Rgb(143, 188, 187))
            }
            StatusOfPlayingItem::Waiting => {
                Line::styled(format!("{}", path_str), Color::Rgb(216, 222, 233))
            }
            StatusOfPlayingItem::Stop => {
                Line::styled(format!("󰓛 {}", path_str), Color::Rgb(143, 188, 187))
            }
        };
        ListItem::new(line)
    }
}

fn display_time(number_seconds: u64) -> String {
    let hours = if number_seconds > 3600 {
        let hours_pre = (number_seconds / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "".to_string()
    };

    let minutes = if number_seconds > 60 {
        let minutes_pre = (number_seconds / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "".to_string()
    };

    let seconds = (number_seconds % 60).to_string() + "s";
    format!("{hours} {minutes} {seconds}")
        .trim_start()
        .to_owned()
}
