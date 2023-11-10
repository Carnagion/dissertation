pub mod aircraft;
pub use aircraft::{Aircraft, Model, Registration, SizeClass};

mod op;
pub use op::{Op, OpKind};

mod separation;
pub use separation::{SeparationId, SeparationMatrix, SeparationRow};

pub struct Instance {
    pub aircraft: Vec<Aircraft>,
    pub operations: Vec<Op>,
    pub separations: SeparationMatrix,
}
