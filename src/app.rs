use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    widgets::ListState,
    DefaultTerminal,
};

use crate::file::get_entrys;
use crate::helper;
use crate::music::{get_song_length, MusicHandle};

pub struct App {
    pub should_exit: bool,
    pub file_list_index_current_display: usize,
    pub playing_list: MusicPlayingList,
    pub inputmode: InputMode,
    pub musichandle: MusicHandle,
    pub musicfile_of_dir: MusicfileOfDir,
    pub apptab: AppTab,
    pub control_table: helper::HelpTable,
}

#[derive(Clone, Copy)]
pub enum InputMode {
    Filelist,
    Playinglist,
    Helper,
}

pub struct MusicFileList {
    pub items: Vec<Musicfile>,
    pub state: ListState,
    pub last_selected: i64,
}

pub struct Musicfile {
    pub info: PathBuf,
    pub status: StatusOfMusicFile,
    pub num_added: u8,
}

pub enum StatusOfMusicFile {
    Added,
    NotAdded,
}

pub struct MusicfileOfDir {
    pub file_lists_of_dir: Vec<MusicFileList>,
    pub map_of_dir_index: HashMap<PathBuf, usize>,
}

pub struct MusicPlayingList {
    pub items: Vec<PlayingItem>,
    pub state: ListState,
    pub playing_music_index: i64,
    pub last_selected: i64,
    pub playingmod: PlayingMod,
    pub total_time: u64,
}

pub struct PlayingItem {
    pub path_of_music: PathBuf,
    pub status: StatusOfPlayingItem,
    pub index_in_dir_and_file: (usize, usize),
    pub length: u32,
}
pub enum StatusOfPlayingItem {
    Playing,
    Pause,
    Waiting,
    Stop,
}

pub enum PlayingMod {
    Auto,
    Repeat,
    Random,
    Manual,
}

pub enum AppTab {
    Music,
    Helper,
}

impl AppTab {
    pub fn next(&self) -> Self {
        match self {
            Self::Music => Self::Helper,
            // Wrap around to the first tab.
            Self::Helper => Self::Music,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        // let folder_path = "/home/charles/Music/demo";
        let current_path = env::current_dir().unwrap();
        let folder_path = current_path;
        let files_path_vec = get_entrys(&Path::new(&folder_path));

        let mut file_lists_dir = Vec::new();
        let file_list = MusicFileList::from_iter(files_path_vec.into_iter());
        file_lists_dir.push(file_list);

        let mut hash_map_dir_index = HashMap::new();
        hash_map_dir_index.insert(PathBuf::from(folder_path), 0 as usize);

        Self {
            should_exit: false,
            playing_list: MusicPlayingList {
                items: Vec::new(),
                state: ListState::default(),
                playing_music_index: -1,
                last_selected: -1,
                playingmod: PlayingMod::Manual,
                total_time: 0,
            },
            inputmode: InputMode::Filelist,
            musichandle: MusicHandle::new(),
            musicfile_of_dir: MusicfileOfDir {
                file_lists_of_dir: file_lists_dir,
                map_of_dir_index: hash_map_dir_index,
            },
            file_list_index_current_display: 0,
            apptab: AppTab::Music,
            control_table: helper::HelpTable::new(),
        }
    }
}

impl FromIterator<PathBuf> for MusicFileList {
    fn from_iter<I: IntoIterator<Item = PathBuf>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            // .filter(|x| crate::file::check_audio_file(x).unwrap())
            .map(|info| Musicfile::new(StatusOfMusicFile::NotAdded, info))
            .collect();
        let state = ListState::default();
        Self {
            items,
            state,
            last_selected: -1,
        }
    }
}

impl Musicfile {
    fn new(status: StatusOfMusicFile, info: PathBuf) -> Self {
        Self {
            status,
            info,
            num_added: 0,
        }
    }
}

impl App {
    pub(crate) fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 2.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.inputmode {
                        InputMode::Playinglist => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                            KeyCode::Char('h') | KeyCode::Left => {
                                self.swith_from_playinglist_to_filelist()
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                self.playing_list.state.select_next()
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                self.playing_list.state.select_previous()
                            }
                            KeyCode::Char('g') => self.playing_list.state.select_first(),
                            KeyCode::Char('G') => self.playing_list.state.select_last(),
                            KeyCode::Enter => self.playing_current_music(),
                            KeyCode::Char('p') => self.swith_playing_and_pause(),
                            KeyCode::Char('s') => self.stop_playing(),
                            KeyCode::Char('n') => self.playing_next_music(),
                            KeyCode::Char('d') => self.remove_slow(),
                            KeyCode::Char('D') => self.remove_fast(),
                            KeyCode::Char('m') => self.change_playing_mod(),
                            KeyCode::Char('-') => self.musichandle.change_volume(-0.05),
                            KeyCode::Char('+') => self.musichandle.change_volume(0.05),
                            KeyCode::Tab => {
                                self.apptab = AppTab::Helper;
                                self.inputmode = InputMode::Helper;
                                self.control_table.last_mod = InputMode::Playinglist;
                            }
                            _ => {}
                        },
                        InputMode::Filelist => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                            KeyCode::Char('j') | KeyCode::Down => self
                                .musicfile_of_dir
                                .file_lists_of_dir[self.file_list_index_current_display]
                                .state
                                .select_next(),
                            KeyCode::Char('k') | KeyCode::Up => self
                                .musicfile_of_dir
                                .file_lists_of_dir[self.file_list_index_current_display]
                                .state
                                .select_previous(),
                            KeyCode::Char('g') => self.musicfile_of_dir.file_lists_of_dir
                                [self.file_list_index_current_display]
                                .state
                                .select_first(),
                            KeyCode::Char('G') => self.musicfile_of_dir.file_lists_of_dir
                                [self.file_list_index_current_display]
                                .state
                                .select_last(),
                            KeyCode::Char('l') | KeyCode::Right => {
                                self.swith_from_filelist_to_playinglist()
                            }
                            KeyCode::Char('a') | KeyCode::Enter => self.add_music_to_playlist(),
                            KeyCode::Char('A') => self.add_all_music_in_current_dir_to_playlist(),
                            KeyCode::Char('o') => self.opendir(),
                            KeyCode::Backspace => self.backdir(),
                            KeyCode::Tab => {
                                self.apptab = AppTab::Helper;
                                self.inputmode = InputMode::Helper;
                                self.control_table.last_mod = InputMode::Filelist;
                            }
                            _ => {}
                        },
                        InputMode::Helper => match key.code {
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.control_table.state.select_next()
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                self.control_table.state.select_previous()
                            }

                            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Tab => {
                                self.apptab = AppTab::Music;
                                self.inputmode = self.control_table.last_mod;
                            }

                            _ => {}
                        },
                    }
                }
            }
        }
        Ok(())
    }

    fn opendir(&mut self) {
        let music_list_display =
            &self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display];
        if let Some(i) = music_list_display.state.selected() {
            let dir = music_list_display.items[i].info.clone();

            if dir.is_dir() {
                let index = self.musicfile_of_dir.map_of_dir_index.get(&dir);
                match index {
                    Some(idx) => self.file_list_index_current_display = *idx,
                    None => {
                        let entrys = get_entrys(&dir);
                        let new_files = MusicFileList::from_iter(entrys);
                        self.musicfile_of_dir.file_lists_of_dir.push(new_files);
                        let index_of_this_list = self.musicfile_of_dir.file_lists_of_dir.len() - 1;
                        self.musicfile_of_dir
                            .map_of_dir_index
                            .insert(PathBuf::from(dir), index_of_this_list);
                        self.file_list_index_current_display = index_of_this_list;
                    }
                }
            }
        }
    }
    fn backdir(&mut self) {
        let music_list_display =
            &self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display];
        if let Some(i) = music_list_display.state.selected() {
            let dir = music_list_display.items[i].info.clone();
            let lastdir = dir.parent().unwrap().parent().unwrap();

            if lastdir.is_dir() {
                let index = self.musicfile_of_dir.map_of_dir_index.get(lastdir);
                match index {
                    Some(idx) => self.file_list_index_current_display = *idx,
                    None => {
                        let entrys = get_entrys(&lastdir);
                        let new_files = MusicFileList::from_iter(entrys);
                        self.musicfile_of_dir.file_lists_of_dir.push(new_files);
                        let index_of_this_list = self.musicfile_of_dir.file_lists_of_dir.len() - 1;
                        self.musicfile_of_dir
                            .map_of_dir_index
                            .insert(PathBuf::from(lastdir), index_of_this_list);
                        self.file_list_index_current_display = index_of_this_list;
                    }
                }
            }
        }
    }

    fn add_all_music_in_current_dir_to_playlist(&mut self) {
        let len = self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display]
            .items
            .len();

        for i in 0..len {
            let path_of_current_music: PathBuf = self.musicfile_of_dir.file_lists_of_dir
                [self.file_list_index_current_display]
                .items[i]
                .info
                .clone();
            match crate::file::check_audio_file(&path_of_current_music) {
                Ok(val) => {
                    if !val {
                        continue;
                    }
                }
                Err(_) => continue,
            }

            self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display].items
                [i]
                .num_added += 1;
            let play_time_of_current_music = match get_song_length(&path_of_current_music) {
                Some(value) => value,
                None => 0,
            };
            self.playing_list.items.push(PlayingItem {
                path_of_music: path_of_current_music,
                status: StatusOfPlayingItem::Waiting,
                index_in_dir_and_file: (self.file_list_index_current_display, i),
                length: play_time_of_current_music,
            });
            self.playing_list.total_time += play_time_of_current_music as u64;

            let item_status = &mut self.musicfile_of_dir.file_lists_of_dir
                [self.file_list_index_current_display]
                .items[i]
                .status;
            *item_status = match *item_status {
                StatusOfMusicFile::NotAdded => StatusOfMusicFile::Added,
                StatusOfMusicFile::Added => StatusOfMusicFile::Added,
            }
        }
    }

    fn add_music_to_playlist(&mut self) {
        let music_list_display =
            &self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display];
        if let Some(i) = music_list_display.state.selected() {
            let path_of_current_music: PathBuf = music_list_display.items[i].info.clone();
            match crate::file::check_audio_file(&path_of_current_music) {
                Ok(val) => {
                    if !val {
                        return;
                    }
                }
                Err(_) => return,
            }

            self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display].items
                [i]
                .num_added += 1;
            let play_time_of_current_music = match get_song_length(&path_of_current_music) {
                Some(value) => value,
                None => 0,
            };
            self.playing_list.items.push(PlayingItem {
                path_of_music: path_of_current_music,
                status: StatusOfPlayingItem::Waiting,
                index_in_dir_and_file: (self.file_list_index_current_display, i),
                length: play_time_of_current_music,
            });
            self.playing_list.total_time += play_time_of_current_music as u64;

            let item_status = &mut self.musicfile_of_dir.file_lists_of_dir
                [self.file_list_index_current_display]
                .items[i]
                .status;
            *item_status = match *item_status {
                StatusOfMusicFile::NotAdded => StatusOfMusicFile::Added,
                StatusOfMusicFile::Added => StatusOfMusicFile::Added,
            }
        }
    }

    fn swith_from_playinglist_to_filelist(&mut self) {
        self.inputmode = InputMode::Filelist;
        if let Some(i) = self.playing_list.state.selected() {
            self.playing_list.last_selected = i as i64;
        }
        self.playing_list.state.select(None);
        let music_list_display =
            &mut self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display];

        if music_list_display.last_selected != -1 {
            music_list_display
                .state
                .select(Some(music_list_display.last_selected as usize));
        }
    }
    fn swith_from_filelist_to_playinglist(&mut self) {
        self.inputmode = InputMode::Playinglist;
        let music_list_display =
            &mut self.musicfile_of_dir.file_lists_of_dir[self.file_list_index_current_display];

        if let Some(i) = music_list_display.state.selected() {
            music_list_display.last_selected = i as i64;
        }
        music_list_display.state.select(None);
        if self.playing_list.last_selected != -1 {
            self.playing_list
                .state
                .select(Some(self.playing_list.last_selected as usize));
        }
    }

    fn playing_current_music(&mut self) {
        if let Some(i) = self.playing_list.state.selected() {
            self.musichandle
                .play_new(self.playing_list.items[i].path_of_music.clone());
            if self.playing_list.playing_music_index != -1 {
                self.playing_list.items[self.playing_list.playing_music_index as usize].status =
                    StatusOfPlayingItem::Waiting;
            }
            self.playing_list.items[i].status = StatusOfPlayingItem::Playing;
            self.playing_list.playing_music_index = i as i64;
        }
    }

    fn swith_playing_and_pause(&mut self) {
        let playing_music_index = self.playing_list.playing_music_index;
        if playing_music_index != -1 {
            self.musichandle.play_pause();
            self.playing_list.items[playing_music_index as usize].status =
                match self.playing_list.items[playing_music_index as usize].status {
                    StatusOfPlayingItem::Playing => StatusOfPlayingItem::Pause,
                    StatusOfPlayingItem::Pause => StatusOfPlayingItem::Playing,
                    StatusOfPlayingItem::Waiting => StatusOfPlayingItem::Waiting,
                    StatusOfPlayingItem::Stop => StatusOfPlayingItem::Stop,
                };
        }
    }

    fn stop_playing(&mut self) {
        let playing_music_index = self.playing_list.playing_music_index;
        if playing_music_index != -1 {
            self.musichandle.stop();
            self.musichandle.set_time_played(0);
            self.playing_list.items[playing_music_index as usize].status =
                StatusOfPlayingItem::Waiting;
        }
        self.playing_list.playing_music_index = -1;
        // self.start_time_of_music = None;
    }

    fn playing_next_music(&mut self) {
        let playing_music_index = self.playing_list.playing_music_index;
        if !self.musichandle.is_empty() {
            if playing_music_index != -1 {
                self.musichandle.stop();
                self.playing_list.items[playing_music_index as usize].status =
                    StatusOfPlayingItem::Waiting;
            }
        }

        let mut next_index = playing_music_index + 1;
        if playing_music_index != -1 {
            if playing_music_index as i64 == self.playing_list.items.len() as i64 - 1 {
                next_index = 0;
            }

            self.musichandle.play_new(
                self.playing_list.items[next_index as usize]
                    .path_of_music
                    .clone(),
            );
            self.playing_list.items[playing_music_index as usize].status =
                StatusOfPlayingItem::Waiting;
            self.playing_list.items[next_index as usize].status = StatusOfPlayingItem::Playing;
            self.playing_list.playing_music_index = next_index;
        } else {
            next_index = 0;
            self.musichandle.play_new(
                self.playing_list.items[next_index as usize]
                    .path_of_music
                    .clone(),
            );
            self.playing_list.items[next_index as usize].status = StatusOfPlayingItem::Playing;
            self.playing_list.playing_music_index = next_index;
        }
    }

    fn remove_slow(&mut self) {
        if let Some(i) = self.playing_list.state.selected() {
            let playing_music_index = self.playing_list.playing_music_index;
            let file_index = self.playing_list.items[i].index_in_dir_and_file;
            if playing_music_index == i as i64 {
                self.stop_playing();
                self.playing_list.items.remove(i);
            } else if playing_music_index < i as i64 {
                self.playing_list.items.remove(i);
            } else if playing_music_index > i as i64 {
                self.playing_list.items.remove(i);
                self.playing_list.playing_music_index -= 1;
            } else if playing_music_index == -1 {
                self.playing_list.items.remove(i);
            }

            let music_item =
                &mut self.musicfile_of_dir.file_lists_of_dir[file_index.0].items[file_index.1];

            music_item.num_added -= 1;

            if music_item.num_added == 0 {
                music_item.status = StatusOfMusicFile::NotAdded;
            }
        }
    }
    fn remove_fast(&mut self) {
        if let Some(i) = self.playing_list.state.selected() {
            let playing_music_index = self.playing_list.playing_music_index;
            let file_index = self.playing_list.items[i].index_in_dir_and_file;
            if playing_music_index == i as i64 {
                self.stop_playing();
                self.playing_list.items.swap_remove(i);
            } else if playing_music_index < i as i64 {
                self.playing_list.items.swap_remove(i);
            } else if playing_music_index > i as i64 {
                let last_index = self.playing_list.items.len() - 1;
                self.playing_list.items.swap_remove(i);
                if playing_music_index == last_index as i64 {
                    self.playing_list.playing_music_index = i as i64;
                }
            } else if playing_music_index == -1 {
                self.playing_list.items.swap_remove(i);
            }

            let music_item =
                &mut self.musicfile_of_dir.file_lists_of_dir[file_index.0].items[file_index.1];

            music_item.num_added -= 1;

            if music_item.num_added == 0 {
                music_item.status = StatusOfMusicFile::NotAdded;
            }
        }
    }

    fn change_playing_mod(&mut self) {
        self.playing_list.playingmod = match self.playing_list.playingmod {
            PlayingMod::Auto => PlayingMod::Repeat,
            PlayingMod::Repeat => PlayingMod::Random,
            PlayingMod::Random => PlayingMod::Manual,
            PlayingMod::Manual => PlayingMod::Auto,
        };
    }

    pub fn is_stop(&mut self) -> bool {
        if self.musichandle.is_empty() && self.playing_list.playing_music_index != -1 {
            true
        } else {
            false
        }
    }

    pub fn handle_stop_music(&mut self) {
        let playing_music_index = self.playing_list.playing_music_index;
        self.playing_list.items[playing_music_index as usize].status = StatusOfPlayingItem::Stop;
    }

    pub fn song_progress(&mut self) -> f64 {
        if self.musichandle.is_empty() && self.playing_list.items.len() == 0 {
            0.0
        } else if !self.musichandle.is_empty() && self.playing_list.playing_music_index != -1 {
            let playing_music_index = self.playing_list.playing_music_index;
            if self.playing_list.items[playing_music_index as usize].length == 0 {
                0.0
            } else {
                f64::clamp(
                    self.musichandle.time_played() as f64
                        / self.playing_list.items[playing_music_index as usize].length as f64,
                    0.0,
                    1.0,
                )
            }
        } else {
            match self.playing_list.playingmod {
                PlayingMod::Auto => {
                    self.auto_play();
                    0.0
                }
                PlayingMod::Manual => 0.0,
                PlayingMod::Repeat => {
                    self.repeat_one_song();
                    0.0
                }
                PlayingMod::Random => {
                    self.random_song();
                    0.0
                }
            }
        }
    }
    fn auto_play(&mut self) {
        // thread::sleep(Duration::from_millis(250));
        if self.musichandle.is_empty() && self.playing_list.items.len() != 0 {
            self.musichandle.set_time_played(0);
            self.playing_next_music();
        }
    }

    fn repeat_one_song(&mut self) {
        if self.musichandle.is_empty() && self.playing_list.items.len() != 0 {
            self.musichandle.set_time_played(0);
            self.playing_same_music();
        }
    }

    fn random_song(&mut self) {
        if self.musichandle.is_empty() && self.playing_list.items.len() != 0 {
            self.musichandle.set_time_played(0);
            self.playing_random_music();
        }
    }

    fn playing_same_music(&mut self) {
        let playing_music_index = self.playing_list.playing_music_index;
        if !self.musichandle.is_empty() {
            if playing_music_index != -1 {
                self.musichandle.stop();
                self.playing_list.items[playing_music_index as usize].status =
                    StatusOfPlayingItem::Waiting;
            }
        }

        let mut next_index = playing_music_index;
        if playing_music_index != -1 {
            self.musichandle.play_new(
                self.playing_list.items[next_index as usize]
                    .path_of_music
                    .clone(),
            );
            self.playing_list.items[next_index as usize].status = StatusOfPlayingItem::Playing;
        } else {
            next_index = 0;
            self.musichandle.play_new(
                self.playing_list.items[next_index as usize]
                    .path_of_music
                    .clone(),
            );
            self.playing_list.items[next_index as usize].status = StatusOfPlayingItem::Playing;
            self.playing_list.playing_music_index = next_index;
        }
    }

    fn playing_random_music(&mut self) {
        let playing_music_index = self.playing_list.playing_music_index;
        if !self.musichandle.is_empty() {
            if playing_music_index != -1 {
                self.musichandle.stop();
                self.playing_list.items[playing_music_index as usize].status =
                    StatusOfPlayingItem::Waiting;
            }
        }

        let upper_bound = self.playing_list.items.len();
        let mut rng = rand::thread_rng();
        let next_index = rand::Rng::gen_range(&mut rng, 0..upper_bound);

        if playing_music_index != -1 {
            self.playing_list.items[playing_music_index as usize].status =
                StatusOfPlayingItem::Waiting;
        }
        self.musichandle.play_new(
            self.playing_list.items[next_index as usize]
                .path_of_music
                .clone(),
        );
        self.playing_list.items[next_index as usize].status = StatusOfPlayingItem::Playing;
        self.playing_list.playing_music_index = next_index as i64;
    }
}
