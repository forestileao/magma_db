use serde::{Deserialize, Serialize};

use super::constants::NODE_HEADER_SIZE;
use super::helpers::create_data_space_offset;

pub type PageSpace = u32;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
enum NodeType {
    Internal,
    Leaf,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct NodeHeader {
    node_type: NodeType,
    free_space_start_offset: PageSpace,
    free_space_end_offset: PageSpace,
    elements_count: PageSpace,
}


impl NodeHeader {
    pub fn new(node_type: NodeType, page_size: PageSpace) -> NodeHeader {
        NodeHeader {
            node_type,
            free_space_start_offset: NODE_HEADER_SIZE as PageSpace,
            free_space_end_offset: page_size,
            elements_count: 0,
        }
    }
}

// ----------------------------------------------------------------
// LeafNode
// ----------------------------------------------------------------

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct LeafNode {
    header: NodeHeader,
    data_space: Vec<u8>,
}

impl LeafNode {
    pub fn new(size: PageSpace) -> LeafNode {
        let header = NodeHeader::new(NodeType::Leaf, size);
        let data_space_offset = create_data_space_offset(header.free_space_end_offset) as usize;
        LeafNode {
            data_space: vec![0_u8; data_space_offset],
            header,
        }
    }
}
