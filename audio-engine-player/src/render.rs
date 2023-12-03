use audio_engine_common::song_time::SongTime;

pub trait Render {
    fn get_sample(&self, song_time: SongTime) -> f32;
}
