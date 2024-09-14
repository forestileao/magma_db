/// The size of a page in bytes.
/// It depends on virtual memory page size of the OS.
pub const PAGE_SIZE: u32 = 4096;

/// To maintain compatibility and prevent restructuring, we allocate extra bytes
/// for the Database header. This reserved space provides room for potential future
/// modifications without altering the existing layout.
pub const DATABASE_HEADER_SIZE: usize = 100;

/// Defines the magic header string used to verify that the loaded
/// file is a valid database file for this engine. This string acts
/// as a unique identifier for files created by this specific database
/// system, helping to prevent misinterpretation of other file types.
pub const MAGIC_HEADER_STRING: [u8; 22] = *b"MagmaDB Storage Engine";
