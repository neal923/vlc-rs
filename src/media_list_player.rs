// Copyright (c) 2015 T. Okub
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use vlc_sys as sys;
use crate::Instance;
use crate::Media;
use crate::MediaList;
use crate::MediaPlayer;
use crate::EventManager;
use crate::PlaybackMode;
use crate::enums::{State};

// A LibVLC media list player plays multiple media from a medialist
pub struct MediaListPlayer {
    pub(crate) ptr: *mut sys::libvlc_media_list_player_t,
}

unsafe impl Send for MediaListPlayer {}

impl MediaListPlayer {
    // Create an empty Media List Player object
    pub fn new(instance: &Instance) -> Option<MediaListPlayer> {
        unsafe{
            let p = sys::libvlc_media_list_player_new(instance.ptr);

            if p.is_null() {
                return None;
            }
            Some(MediaListPlayer{ptr: p})
        }
    }

    // Get the Event Manager from which the media list player sends events
    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = sys::libvlc_media_list_player_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    // Set the media player that will be used by the media list player.
    pub fn set_media_player(&self, mdp: &MediaPlayer) {
        unsafe{ sys::libvlc_media_list_player_set_media_player(self.ptr, mdp.ptr) };
    }

    // Get the media player used by the media list player.
    pub fn get_media_player(&self) -> Option<MediaPlayer> {
        let p = unsafe{ sys::libvlc_media_list_player_get_media_player(self.ptr) };
        if p.is_null() {
            None
        }else{
            Some(MediaPlayer{ptr: p})
        }
    }

    // Set the media list that will be used by the media list player.
    pub fn set_media_list(&self, ml: &MediaList) {
        unsafe{ sys::libvlc_media_list_player_set_media_list(self.ptr, ml.ptr) };
    }

    // Play media list
    pub fn play(&self) -> Result<(), ()> {
        unsafe { 
            sys::libvlc_media_list_player_play(self.ptr);
            Ok(())
        }
    }

    // Toggle pause (or resume) media list
    pub fn pause(&self) {
        unsafe{ sys::libvlc_media_list_player_pause(self.ptr) };
    }

    // Pause or resume media list
    pub fn set_pause(&self, do_pause: bool) {
        unsafe{ sys::libvlc_media_list_player_set_pause(self.ptr, if do_pause {1} else {0}) };
    }

    // Is media list playing?
    pub fn is_playing(&self) -> bool {
        if unsafe{ sys::libvlc_media_list_player_is_playing(self.ptr) } == 0 {
            false
        }else{
            true
        }
    }

    // Get current libvlc_state of media list player.
    pub fn state(&self) -> State {
        unsafe{ sys::libvlc_media_list_player_get_state(self.ptr) }.into()
    }

    // Play media list item at position index
    pub fn play_item_at_index(&self, index: i32) -> Result<(), ()> {
        if unsafe { sys::libvlc_media_list_player_play_item_at_index(self.ptr, index)}  == 0 {
            Ok(())
        }else{
            Err(())
        }
    }

    // Set the media that will be used by the media_player. If any, previous md will be released.
    pub fn play_item(&self, md: &Media) -> Result<(), ()> {
        if unsafe{ sys::libvlc_media_list_player_play_item(self.ptr, md.ptr) } == 0 {
            Ok(())
        }else{
            Err(())
        }
    }

    // Stop (no effect if there is no media)
    pub fn stop(&self) {
        unsafe{ sys::libvlc_media_list_player_stop(self.ptr) };
    }

    // Play next item from media list
    pub fn next(&self) -> Result<(), ()> {
        if unsafe{ sys::libvlc_media_list_player_next(self.ptr) } == 0 {
            Ok(())
        }else{
            Err(())
        }
    }

    // Play previous item from media list
    pub fn previous(&self) -> Result<(), ()> {
        if unsafe{ sys::libvlc_media_list_player_previous(self.ptr) } == 0 {
            Ok(())
        }else{
            Err(())
        }
    }

    // Sets the playback mode for the playlist
    pub fn set_playback_mode(&self, mode: PlaybackMode) {
        unsafe{ sys::libvlc_media_list_player_set_playback_mode(self.ptr, mode as u32) };
    }

    // Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_list_player_t {
        self.ptr
    }
}

impl Drop for MediaListPlayer {
    fn drop(&mut self) {
        unsafe{ sys::libvlc_media_list_player_release(self.ptr) };
    }
}
