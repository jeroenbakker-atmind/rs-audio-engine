use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    song_time::SongTime,
};
use audio_engine_effect::effect::Effect;
use audio_engine_sequencer::instrument::InstrumentID;

use crate::{
    event::Event, row::Row, song::Song, song_state::SongState, track::Track,
    track_state::TrackState,
};

pub struct Tracker {
    pub song: Song,
    pub song_state: SongState,

    /// Output sample rate in hz (44100.0)
    pub sample_rate: f32,
}

impl Tracker {
    pub fn render(&mut self) -> Vec<f32> {
        let mut result = Vec::default();

        self.song_state = self.song.init_sound_state();

        let mut sample_number = 0;
        while let Some(sample) = sample_song(
            &self.song,
            &mut self.song_state,
            sample_number as f32 / self.sample_rate,
            self.sample_rate,
        ) {
            result.push(sample);
            sample_number += 1;
        }

        result
    }
}

pub fn sample_song(
    song: &Song,
    song_state: &mut SongState,
    song_time: SongTime,
    sample_rate: f32,
) -> Option<f32> {
    let mut result = None;

    const SECONDS_PER_MINUTE: f32 = 60.0;

    let beats_per_second = song.speed / SECONDS_PER_MINUTE;
    let global_row_index = (song_time * beats_per_second * song_state.rows_per_beat) as u32;

    for track_id in 0..8 {
        let track = &song.tracks[track_id];
        if let Some(row) = calc_track_position(song_state, song, track, global_row_index) {
            let track_state = &mut song_state.tracks[track_id];
            apply_row(song, track_state, song_time, global_row_index, row);
            let track_result = sample_track(song, track, track_state, song_time, sample_rate);
            match (result, track_result) {
                (None, sample) => result = Some(sample),
                (Some(a), sample) => {
                    result = Some(a + sample);
                }
            }
        }
    }

    result
}

pub fn sample_track(
    song: &Song,
    track: &Track,
    track_state: &mut TrackState,
    song_time: SongTime,
    sample_rate: f32,
) -> f32 {
    let track_sample = if let Some(note_on) = track_state.note_on {
        if let Some(instrument) = song.get_instrument(track_state.instrument_id) {
            let note_time = song_time - note_on;
            let note_off = track_state.note_off.map(|note_off| song_time - note_off);
            instrument.sample(
                &NoteParameters {
                    note_time,
                    note_off,
                    note_pitch: track_state.note_pitch,
                    gain: track_state.level,
                    sample_rate,
                },
                &mut track_state.instrument_note_state,
            ) * track.level
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Apply track effects.
    let mut track_sample = [track_sample];
    apply_track_effects(&mut track_sample, sample_rate, track, track_state);

    track_sample[0]
}

pub fn apply_track_effects(
    track_samples: &mut [f32],
    sample_rate: f32,
    track: &Track,
    track_state: &mut TrackState,
) {
    track
        .delay
        .effect_apply(track_samples, sample_rate, &mut track_state.delay_state);
    track.distortion.effect_apply(
        track_samples,
        sample_rate,
        &mut track_state.distortion_state,
    );
}

pub fn calc_track_position<'a>(
    song_state: &mut SongState,
    song: &'a Song,
    track: &'a Track,
    global_row_index: u32,
) -> Option<&'a Row> {
    let mut rows_left = global_row_index;
    for phrase_id in track.phrases {
        if let Some(phrase) = song.get_phrase(phrase_id) {
            let phrase_row_count = song_state.get_phrase_row_len(phrase_id);
            if phrase_row_count <= rows_left {
                rows_left -= phrase_row_count;
                continue;
            }
            for pattern_id in phrase.patterns {
                if let Some(pattern) = song.get_pattern(pattern_id) {
                    let pattern_row_count = song_state.get_pattern_row_len(pattern_id);
                    if pattern_row_count <= rows_left {
                        rows_left -= pattern_row_count;
                        continue;
                    }
                    return Some(&pattern.rows[rows_left as usize]);
                }
            }
        } else {
            break;
        }
    }
    None
}

fn assign_if_different<T>(a: &mut T, b: &T) -> bool
where
    T: PartialEq + Copy,
{
    if a != b {
        *a = *b;
        true
    } else {
        false
    }
}

fn apply_row(
    song: &Song,
    track_state: &mut TrackState,
    song_time: SongTime,
    global_row_index: u32,
    row: &Row,
) {
    let is_new_row = assign_if_different(&mut track_state.global_row_index, &global_row_index);
    if is_new_row {
        //     println!("{}", row);
        match row.event {
            Some(Event::NoteOn(note, instrument_id)) => {
                track_state.note_on = Some(song_time);
                if instrument_id != InstrumentID::NotSet {
                    track_state.instrument_id = instrument_id;
                }

                track_state.note_pitch = note.pitch();
                let instrument = song.get_instrument(track_state.instrument_id);
                track_state.instrument_note_state.reset(instrument);
            }
            Some(Event::NoteRelease) => {
                track_state.note_off = Some(song_time);
            }
            Some(Event::NoteOff) => {
                track_state.note_on = None;
                track_state.note_off = None;
                track_state.instrument_note_state.reset(None);
            }
            Some(Event::Empty) | Some(Event::PatternEnd) | None => {}
        }

        if let Some(level) = row.level {
            track_state.level = level;
        }
    }
}
