// SPDX-License-Identifier: Apache-2.0

use super::Argv;
use crate::guest::alloc::{
    Allocator, Collect, Collector, Commit, CommittedCall, Committer, PassthroughSyscall, StagedCall,
};
use crate::guest::Call;
use crate::Result;

use libc::{
    c_int, c_long, EBADFD, EINVAL, F_GETFD, F_GETFL, F_SETFD, F_SETFL, O_APPEND, O_RDWR, O_WRONLY,
    STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO,
};

pub struct Fcntl {
    pub fd: c_int,
    pub cmd: c_int,
    pub arg: c_int,
}

pub struct Alloc(Fcntl);

unsafe impl PassthroughSyscall for Alloc {
    const NUM: c_long = libc::SYS_fcntl;

    type Argv = Argv<3>;
    type Ret = c_int;

    fn stage(self) -> Self::Argv {
        Argv([self.0.fd as _, self.0.cmd as _, self.0.arg as _])
    }
}

pub enum StagedFcntl<'a> {
    Alloc(StagedCall<'a, Alloc>),
    Stub(c_int),
}

impl<'a> Call<'a> for Fcntl {
    type Staged = StagedFcntl<'a>;
    type Committed = CommittedFcntl<'a>;
    type Collected = Result<c_int>;

    fn stage(self, alloc: &mut impl Allocator) -> Result<Self::Staged> {
        match (self.fd, self.cmd) {
            (STDIN_FILENO, F_GETFL) => Ok(StagedFcntl::Stub(O_RDWR | O_APPEND)),
            (STDOUT_FILENO | STDERR_FILENO, F_GETFL) => Ok(StagedFcntl::Stub(O_WRONLY)),
            (STDIN_FILENO | STDOUT_FILENO | STDERR_FILENO, _) => Err(EINVAL),
            (_, F_GETFD | F_SETFD | F_GETFL | F_SETFL) => {
                Call::stage(Alloc(self), alloc).map(StagedFcntl::Alloc)
            }
            (_, _) => Err(EBADFD),
        }
    }
}

pub enum CommittedFcntl<'a> {
    Alloc(CommittedCall<'a, Alloc>),
    Stub(c_int),
}

impl<'a> Commit for StagedFcntl<'a> {
    type Item = CommittedFcntl<'a>;

    fn commit(self, com: &impl Committer) -> CommittedFcntl<'a> {
        match self {
            StagedFcntl::Alloc(call) => CommittedFcntl::Alloc(call.commit(com)),
            StagedFcntl::Stub(val) => CommittedFcntl::Stub(val),
        }
    }
}

impl<'a> Collect for CommittedFcntl<'a> {
    type Item = Result<c_int>;

    fn collect(self, col: &impl Collector) -> Self::Item {
        match self {
            CommittedFcntl::Alloc(call) => Collect::collect(call, col),
            CommittedFcntl::Stub(val) => Ok(val),
        }
    }
}
