use crate::track_state::TrackState;

#[derive(Default)]
pub struct SongState {
    pub tracks: [TrackState; 8],
}
