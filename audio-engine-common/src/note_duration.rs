#[derive(Debug, Default, Copy, Clone)]
pub enum NoteDurationType {
    Full,
    Half,
    #[default]
    Quarter,
    Eight,
    Sixteen,
    Beat,
}

#[derive(Debug, Copy, Clone)]
pub enum NoteDuration {
    Duration(NoteDurationType, usize),
    Add(NoteDurationType, NoteDurationType),
}

impl Default for NoteDuration {
    fn default() -> Self {
        NoteDuration::Duration(NoteDurationType::default(), 1)
    }
}
