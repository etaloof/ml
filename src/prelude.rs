//! The purpose of this module is to provide reexports of core traits so that they can be then
//! glob-imported all at once:
//!
//! ```
//! use ml::prelude::*;
//! ```

pub use ::DEFAULT_NAME_DOT;
pub use ::DEFAULT_NAME_PNG;
pub use ::syntax::segment::Segment;
pub use ::syntax::item::Item;
pub use ::syntax::item::relation::Relation;
pub use ::syntax::item::state::ItemState;
pub use ::syntax::item::state::method::Method;
pub use ::syntax::item::state::implem::Implem;
pub use ::syntax::item::state::abstraction::Abstract;
pub use ::syntax::item::state::abstraction::extend::Trait;
pub use ::syntax::item::state::abstraction::structure::Struct;
pub use ::syntax::item::state::abstraction::enumerate::Enum;
