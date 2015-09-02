pub mod inout;
pub use self::inout::InOut;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::Context;

pub mod filtergraph;
pub use self::filtergraph::FilterGraph;

use ffi::*;

pub fn register_all() {
	unsafe {
		avfilter_register_all();
	}
}
