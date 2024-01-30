pub use irdis_core::instance;

pub mod solve {
    pub use irdis_core::solve::Solve;

    #[cfg(feature = "branch-bound")]
    pub use irdis_branch_bound::BranchBound;
}

#[cfg(feature = "vis")]
#[doc(inline)]
pub use irdis_vis as vis;
