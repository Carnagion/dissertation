pub use irdis_core::instance;

pub mod solve {
    pub use irdis_core::solve::Solve;

    #[cfg(feature = "branch-bound")]
    #[doc(inline)]
    pub use irdis_branch_bound as branch_bound;
}

#[cfg(feature = "vis")]
#[doc(inline)]
pub use irdis_vis as vis;
