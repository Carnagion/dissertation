pub mod instance {
    #[doc(inline)]
    pub use irdis_instance::{flight, schedule, sep, time, Instance};
}

pub mod solve {
    pub use irdis_instance::Solve;

    #[cfg(feature = "branch-bound")]
    pub use irdis_branch_bound::BranchBound;
}

#[cfg(feature = "vis")]
#[doc(inline)]
pub use irdis_vis as vis;
