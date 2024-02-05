use crate::{builder::Builder, link::Link, link_path::LinkPath, node_index::NodeIndex};

#[derive(Copy, Clone)]
pub struct LinkBuilder {
    link: Link,
}

impl Builder for LinkBuilder {
    type Inner = Link;
    fn new() -> Self {
        LinkBuilder {
            link: Link {
                from_node: 0.into(),
                to_node: 0.into(),
                weight: 1.0,
                path: LinkPath::default(),
            },
        }
    }

    fn build(&self) -> Self::Inner {
        self.link
    }
}

impl LinkBuilder {
    pub fn from_node<N>(&mut self, node_index: N) -> &mut Self
    where
        N: Into<NodeIndex>,
    {
        self.link.from_node = node_index.into();
        self
    }
    pub fn to_node<N>(&mut self, node_index: N) -> &mut Self
    where
        N: Into<NodeIndex>,
    {
        self.link.to_node = node_index.into();
        self
    }

    pub fn weight(&mut self, weight: f64) -> &mut Self {
        self.link.weight = weight;
        self
    }

    pub fn path(&mut self, path: LinkPath) -> &mut Self {
        self.link.path = path;
        self
    }

    pub fn build(self) -> Link {
        self.link
    }
}

impl Into<Link> for LinkBuilder {
    fn into(self) -> Link {
        self.build()
    }
}
