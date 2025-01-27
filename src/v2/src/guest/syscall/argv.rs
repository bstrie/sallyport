// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Argv<const N: usize>(pub [usize; N]);

impl From<Argv<0>> for [usize; 6] {
    #[inline]
    fn from(_: Argv<0>) -> Self {
        [0, 0, 0, 0, 0, 0]
    }
}

impl From<Argv<1>> for [usize; 6] {
    #[inline]
    fn from(argv: Argv<1>) -> Self {
        [argv.0[0], 0, 0, 0, 0, 0]
    }
}

impl From<Argv<2>> for [usize; 6] {
    #[inline]
    fn from(argv: Argv<2>) -> Self {
        [argv.0[0], argv.0[1], 0, 0, 0, 0]
    }
}

impl From<Argv<3>> for [usize; 6] {
    #[inline]
    fn from(argv: Argv<3>) -> Self {
        [argv.0[0], argv.0[1], argv.0[2], 0, 0, 0]
    }
}

impl From<Argv<4>> for [usize; 6] {
    #[inline]
    fn from(argv: Argv<4>) -> Self {
        [argv.0[0], argv.0[1], argv.0[2], argv.0[3], 0, 0]
    }
}

impl From<Argv<5>> for [usize; 6] {
    #[inline]
    fn from(argv: Argv<5>) -> Self {
        [argv.0[0], argv.0[1], argv.0[2], argv.0[3], argv.0[4], 0]
    }
}

impl From<Argv<6>> for [usize; 6] {
    #[inline]
    fn from(argv: Argv<6>) -> Self {
        argv.0
    }
}
