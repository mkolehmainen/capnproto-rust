// Copyright (c) 2026 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use crate::private::capability::ClientHook;

// On platforms without file descriptors these types must exist (so the
// FD-passing API has the same shape everywhere) but must be impossible to
// construct. A private `Infallible` field achieves that; it also lets
// `match self.0 {}` prove every method unreachable.
#[derive(Debug, Clone, Copy)]
pub struct BorrowedFd<'a>(core::convert::Infallible, core::marker::PhantomData<&'a ()>);

#[derive(Debug)]
pub struct OwnedFd(core::convert::Infallible);

pub trait AsFd {
    fn as_fd(&self) -> BorrowedFd<'_>;
}

impl AsFd for BorrowedFd<'_> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self.0 {}
    }
}

impl AsFd for OwnedFd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self.0 {}
    }
}

#[derive(Default)]
#[non_exhaustive]
pub struct FdHooks {}

impl FdHooks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_push(
        &mut self,
        hook: Box<dyn ClientHook>,
    ) -> Result<(u8, &dyn ClientHook), Box<dyn ClientHook>> {
        Err(hook)
    }

    pub fn as_fds(&self) -> &[BorrowedFd<'_>] {
        &[]
    }
}
