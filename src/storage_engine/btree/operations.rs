use log::debug;

use crate::storage_engine::pager::{constants::{METAPAGE_ID, METAPAGE_SIZE}, Pager};

use super::node::LeafNode;

pub fn initialize_root_node_if_needed(pager: &mut Pager) {
    if !pager.is_initialized() {
        debug!("Initializing the root node within the metapage");
        let root_node = create_root_node();

        save_node(pager, &root_node, Some(METAPAGE_ID)).unwrap();
    }
}

fn create_root_node() -> LeafNode {
    LeafNode::new(METAPAGE_SIZE)
}
