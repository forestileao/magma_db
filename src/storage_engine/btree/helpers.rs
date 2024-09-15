use super::node::PageSpace;
use super::constants::NODE_HEADER_SIZE;

pub fn create_data_space_offset(absolute_offset: PageSpace) -> PageSpace {
    absolute_offset - (NODE_HEADER_SIZE as PageSpace)
}
