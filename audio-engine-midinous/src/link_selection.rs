#[derive(Default, Debug, Copy, Clone)]
pub enum LinkSelection {
    #[default]
    Sequential,
    Random,
    All,
}
