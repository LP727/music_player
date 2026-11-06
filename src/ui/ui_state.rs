//! UI States
//! Tracks what is currently displayed on screen to map user interaction with proper command

use crate::library::{Library, Track};

const MENU_OPTIONS: &[&str] = &["All songs", "Artists", "Albums"];

// Statically allocated for lifetime of program, equivalent to returning:
// static PROMOTED: [Footer; 1] = [Footer::MENU];
const MENU_ONLY: &[Footer] = &[Footer::MENU];
const MENU_BACK: &[Footer] = &[Footer::BACK, Footer::MENU];

pub enum UIState {
    BrowseMenu(BrowseMenuState),
    AllSongs(AllSongsState),
    AllArtists(AllArtistsState),
    AllAlbum(AllAlbumState),
    ArtistSongs(ArtistSongsState),
    ArtistAlbums(ArtistAlbumsState),
    AlbumSongs(AlbumSongsState),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Footer {
    BACK,
    MENU,
}

impl Footer {
    fn label(self) -> &'static str {
        match self {
            Footer::BACK => "Back",
            Footer::MENU => "Menu",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Choice {
    All,
    Item(usize),
    Footer(Footer),
}

fn draw<S: AsRef<str>>(title: String, lines: &[S]) {
    let mut num = 1;
    // \x1B[2J clears the screen, \x1B[1;1H moves the cursor to the top-left
    println!("\x1B[2J\x1B[1;1H");
    println!("{title}\r");
    for line in lines {
        println!("{}. {}\r", num, line.as_ref());
        num += 1;
    }
}

pub trait UiDisplay {
    fn title(&self) -> String;
    fn items(&self) -> Vec<String>;
    fn footer(&self) -> &'static [Footer] {
        MENU_ONLY
    }

    fn has_all(&self) -> bool {
        false
    }

    fn lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.has_all() {
            lines.push(String::from("All"));
        }
        lines.extend(self.items().into_iter()); // consumes the String
        lines.extend(self.footer().iter().map(|f| f.label().to_string()));
        lines
    }

    fn resolve(&self, idx: usize) -> Option<Choice> {
        let n = self.items().len();
        if self.has_all() {
            if idx == 0 {
                return Some(Choice::All);
            } else if idx <= n {
                return Some(Choice::Item(idx - 1));
            } else if let Some(ft) = self.footer().get(idx - 1 - n) {
                return Some(Choice::Footer(*ft));
            }
        } else {
            if idx < n {
                return Some(Choice::Item(idx)); // no all options, n must be < than vector len
            } else if let Some(ft) = self.footer().get(idx - n) {
                return Some(Choice::Footer(*ft));
            }
        }
        None
    }

    fn ui_display(&self) {
        draw(self.title(), &self.lines());
    }
}

pub struct BrowseMenuState {}
pub struct AllSongsState {
    songs: Vec<Track>,
}
pub struct AllArtistsState {
    artists: Vec<String>,
}
pub struct AllAlbumState {
    albums: Vec<String>,
}
pub struct ArtistSongsState {
    name: String,
    songs: Vec<Track>,
}
pub struct ArtistAlbumsState {
    name: String,
    albums: Vec<String>,
}
pub struct AlbumSongsState {
    name: String,
    songs: Vec<Track>,
}

impl UIState {
    pub fn new() -> UIState {
        UIState::BrowseMenu(BrowseMenuState {})
    }
}

impl UiDisplay for UIState {
    fn title(&self) -> String {
        match self {
            UIState::AlbumSongs(s) => s.title(),
            UIState::AllAlbum(s) => s.title(),
            UIState::ArtistAlbums(s) => s.title(),
            UIState::ArtistSongs(s) => s.title(),
            UIState::AllArtists(s) => s.title(),
            UIState::AllSongs(s) => s.title(),
            UIState::BrowseMenu(s) => s.title(),
        }
    }
    fn items(&self) -> Vec<String> {
        match self {
            UIState::AlbumSongs(s) => s.items(),
            UIState::AllAlbum(s) => s.items(),
            UIState::ArtistAlbums(s) => s.items(),
            UIState::ArtistSongs(s) => s.items(),
            UIState::AllArtists(s) => s.items(),
            UIState::AllSongs(s) => s.items(),
            UIState::BrowseMenu(s) => s.items(),
        }
    }

    fn has_all(&self) -> bool {
        match self {
            UIState::AlbumSongs(s) => s.has_all(),
            UIState::AllAlbum(s) => s.has_all(),
            UIState::ArtistAlbums(s) => s.has_all(),
            UIState::ArtistSongs(s) => s.has_all(),
            UIState::AllArtists(s) => s.has_all(),
            UIState::AllSongs(s) => s.has_all(),
            UIState::BrowseMenu(s) => s.has_all(),
        }
    }

    fn footer(&self) -> &'static [Footer] {
        match self {
            UIState::AlbumSongs(s) => s.footer(),
            UIState::AllAlbum(s) => s.footer(),
            UIState::ArtistAlbums(s) => s.footer(),
            UIState::ArtistSongs(s) => s.footer(),
            UIState::AllArtists(s) => s.footer(),
            UIState::AllSongs(s) => s.footer(),
            UIState::BrowseMenu(s) => s.footer(),
        }
    }
}

// Main Menu
impl BrowseMenuState {
    pub fn artists<T: Library>(self, lib: &T) -> AllArtistsState {
        let artists = lib.all_artists();
        AllArtistsState { artists }
    }
    pub fn albums<T: Library>(self, lib: &T) -> AllAlbumState {
        let albums = lib.all_albums();
        AllAlbumState { albums }
    }
    pub fn all_songs<T: Library>(self, lib: &T) -> AllSongsState {
        let songs = lib.all_tracks();
        AllSongsState { songs }
    }
}

impl UiDisplay for BrowseMenuState {
    fn title(&self) -> String {
        String::from("Main Menu")
    }

    fn items(&self) -> Vec<String> {
        MENU_OPTIONS.iter().map(|s| s.to_string()).collect()
    }
}

// All songs view
impl AllSongsState {
    pub fn get_all(&self) -> Vec<Track> {
        self.songs.clone()
    }

    pub fn select(&self, selection: usize) -> Track {
        self.songs[selection].clone()
    }

    pub fn menu(self) -> BrowseMenuState {
        BrowseMenuState {}
    }
}

impl UiDisplay for AllSongsState {
    fn title(&self) -> String {
        String::from("All Songs")
    }

    fn items(&self) -> Vec<String> {
        self.songs.iter().map(|s| s.title.clone()).collect()
    }

    fn has_all(&self) -> bool {
        true
    }
}

impl AllArtistsState {
    // Selecting by index, since the display provides the index value
    pub fn select<T: Library>(self, selection: usize, lib: &T) -> ArtistAlbumsState {
        let artist_albums = lib.albums_by_artist(&self.artists[selection]);

        ArtistAlbumsState {
            name: self.artists[selection].clone(),
            albums: artist_albums,
        }
    }

    pub fn menu(self) -> BrowseMenuState {
        BrowseMenuState {}
    }
}

impl UiDisplay for AllArtistsState {
    fn title(&self) -> String {
        String::from("All Artists")
    }

    fn items(&self) -> Vec<String> {
        self.artists.clone()
    }
}

impl AllAlbumState {
    // Selecting by index, since the display provides the index value
    pub fn select<T: Library>(self, selection: usize, lib: &T) -> AlbumSongsState {
        let songs = lib.tracks_by_album(&self.albums[selection]);

        AlbumSongsState {
            name: self.albums[selection].clone(),
            songs,
        }
    }

    pub fn menu(self) -> BrowseMenuState {
        BrowseMenuState {}
    }
}

impl UiDisplay for AllAlbumState {
    fn title(&self) -> String {
        String::from("All Albums")
    }

    fn items(&self) -> Vec<String> {
        self.albums.clone()
    }
}

impl ArtistAlbumsState {
    pub fn get_all<T: Library>(self, lib: &T) -> ArtistSongsState {
        let songs = lib.tracks_by_artist(&self.name);
        ArtistSongsState {
            name: self.name,
            songs: songs,
        }
    }
    // Selecting by index, since the display provides the index value
    pub fn select<T: Library>(self, selection: usize, lib: &T) -> AlbumSongsState {
        let songs = lib.tracks_by_album(&self.albums[selection]);

        AlbumSongsState {
            name: self.albums[selection].clone(),
            songs,
        }
    }

    pub fn back<T: Library>(self, lib: &T) -> AllArtistsState {
        AllArtistsState {
            artists: lib.all_artists(),
        }
    }

    pub fn menu(self) -> BrowseMenuState {
        BrowseMenuState {}
    }
}

impl UiDisplay for ArtistAlbumsState {
    fn title(&self) -> String {
        let title = format!("{} Albums", self.name);
        title
    }

    fn items(&self) -> Vec<String> {
        self.albums.clone()
    }

    fn footer(&self) -> &'static [Footer] {
        MENU_BACK
    }

    fn has_all(&self) -> bool {
        true
    }
}

impl ArtistSongsState {
    pub fn get_all(&self) -> Vec<Track> {
        self.songs.clone()
    }

    pub fn select(&self, selection: usize) -> Track {
        self.songs[selection].clone()
    }

    pub fn back<T: Library>(self, lib: &T) -> AllArtistsState {
        AllArtistsState {
            artists: lib.all_artists(),
        }
    }

    pub fn menu(self) -> BrowseMenuState {
        BrowseMenuState {}
    }
}

impl UiDisplay for ArtistSongsState {
    fn title(&self) -> String {
        let title = format!("{} Songs", self.name);
        title
    }

    fn items(&self) -> Vec<String> {
        self.songs.iter().map(|s| s.title.clone()).collect()
    }

    fn footer(&self) -> &'static [Footer] {
        MENU_BACK
    }

    fn has_all(&self) -> bool {
        true
    }
}

impl AlbumSongsState {
    pub fn get_all(&self) -> Vec<Track> {
        self.songs.clone()
    }

    pub fn select(&self, selection: usize) -> Track {
        self.songs[selection].clone()
    }

    pub fn back_to_all_album<T: Library>(self, lib: &T) -> AllAlbumState {
        AllAlbumState {
            albums: lib.all_albums(),
        }
    }

    pub fn back_to_artist_albums<T: Library>(self, lib: &T) -> ArtistAlbumsState {
        ArtistAlbumsState {
            // TODO: this is bugged, the name here is the album name, needs an origin field or
            // History stack
            name: self.name.clone(),
            albums: lib.albums_by_artist(&self.name),
        }
    }

    pub fn menu(self) -> BrowseMenuState {
        BrowseMenuState {}
    }
}

impl UiDisplay for AlbumSongsState {
    fn title(&self) -> String {
        let title = format!("{} Songs", self.name);
        title
    }

    fn items(&self) -> Vec<String> {
        self.songs.iter().map(|s| s.title.clone()).collect()
    }

    fn footer(&self) -> &'static [Footer] {
        MENU_BACK
    }

    fn has_all(&self) -> bool {
        true
    }
}
