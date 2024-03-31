use audio_engine_discrete_time::filter::Filter;

#[derive(Debug, Clone)]
pub struct PianoString {
    filter: Filter<f64>,
}

impl PianoString {
    pub fn filter(&mut self, value_in: f64) -> f64 {
        self.filter.filter(value_in)
    }
}

impl<F> From<F> for PianoString
where
    F: Into<Filter<f64>> + Sized,
{
    fn from(value: F) -> Self {
        PianoString {
            filter: value.into(),
        }
    }
}
