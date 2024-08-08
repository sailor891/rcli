// 直接暴露process下的所有模块
// 使用时 use rcli::process::csv_convert::process_csv;
// pub mod csv_convert;
// pub mod gen_pass;

// 将process里面的crate暴露出去，其它main使用时可以直接使用 use rcli::process::process_csv;
mod b64;
mod csv_convert;
mod gen_pass;

pub use b64::{process_decode, process_encode};
pub use csv_convert::process_csv;
pub use gen_pass::process_genpass;
