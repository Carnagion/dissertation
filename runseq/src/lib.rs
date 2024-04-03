#![deny(rust_2018_idioms)]

#[doc(inline)]
pub use runseq_instance as instance;

#[cfg(feature = "data")]
#[doc(inline)]
pub use runseq_data as data;

#[cfg(feature = "branch-bound")]
#[doc(inline)]
pub use runseq_branch_bound as branch_bound;

#[cfg(feature = "vis")]
#[doc(inline)]
pub use runseq_vis as vis;
