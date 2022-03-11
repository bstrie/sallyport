// SPDX-License-Identifier: Apache-2.0

use super::super::Alloc;
use crate::guest::alloc::{Collect, Collector, Commit, Committer, InOutRef, InRef, Input, OutRef};
use crate::libc;
use crate::Result;

/// Staged syscall, which holds allocated reference to syscall item within the block and [opaque staged value](Alloc::Staged).
pub struct StagedAlloc<'a, T: Alloc<'a>> {
    pub(crate) num_ref: InRef<'a, usize>,
    pub(crate) argv: Input<'a, [usize; 6], [usize; 6]>,
    pub(crate) ret_ref: InOutRef<'a, [usize; 2]>,
    pub(crate) staged: T::Staged,
}

impl<'a, T: Alloc<'a>> Commit for StagedAlloc<'a, T> {
    type Item = CommittedAlloc<'a, T>;

    #[inline]
    fn commit(mut self, com: &impl Committer) -> Self::Item {
        self.num_ref.copy_from(com, T::NUM as usize);
        self.argv.commit(com);
        self.ret_ref.copy_from(com, [-libc::ENOSYS as usize, 0]);
        Self::Item {
            ret_ref: self.ret_ref.commit(com),
            committed: self.staged.commit(com),
        }
    }
}

/// Committed syscall, which holds allocated reference to syscall return values within the block and [opaque committed value](Alloc::Committed).
pub struct CommittedAlloc<'a, T: Alloc<'a>> {
    pub(crate) ret_ref: OutRef<'a, [usize; 2]>,
    pub(crate) committed: T::Committed,
}

impl<'a, T: Alloc<'a>> Collect for CommittedAlloc<'a, T>
where
    super::Result<T::Ret>: Into<Result<T::Ret>>,
{
    type Item = T::Collected;

    #[inline]
    fn collect(self, col: &impl Collector) -> Self::Item {
        let mut ret = [0usize; 2];
        self.ret_ref.copy_to(col, &mut ret);
        let res: super::Result<T::Ret> = ret.into();
        T::collect(self.committed, res.into(), col)
    }
}
