mod wine;
mod utils;
mod dxvk;

pub use wine::Wine;
pub use dxvk::{get_latest_dxvk, install_dxvk, remove_dxvk};