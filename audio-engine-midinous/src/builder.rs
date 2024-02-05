pub trait Builder {
    type Inner;
    fn new() -> Self;
    fn build(&self) -> Self::Inner;
}

// TODO: move to tests folder as they are integration tests.
#[cfg(test)]
mod test {

    use crate::{builder_link::LinkBuilder, builder_node::NodeBuilder, builder_song::SongBuilder};

    use super::Builder;

    #[test]
    fn empty_song() {
        let song = SongBuilder::new().build();
    }

    #[test]
    fn two_nodes() {
        let mut builder = SongBuilder::new();
        let song = builder
            .entry_point(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .build();
    }

    #[test]
    fn loop_four_nodes() {
        let mut builder = SongBuilder::new();
        let song = builder
            .entry_point(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .link(*LinkBuilder::new().from_node(3).to_node(0))
            .build();
    }
}
