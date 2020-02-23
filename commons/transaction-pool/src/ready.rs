// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Transaction readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Readiness {
    /// The transaction is stale (and should/will be removed from the pool).
    Stale,
    /// The transaction is ready to be included in pending set.
    Ready,
    /// The transaction is not yet ready.
    Future,
}

/// A readiness indicator.
#[async_trait(?Send)]
pub trait Ready<T>
where
    T: Sync,
{
    /// Returns true if transaction is ready to be included in pending block,
    /// given all previous transactions that were ready are already included.
    ///
    /// NOTE: readiness of transactions will be checked according to `Score` ordering,
    /// the implementation should maintain a state of already checked transactions.
    async fn is_ready(&mut self, tx: &T) -> Readiness;
}

#[async_trait(?Send)]
impl<T, F> Ready<T> for F
where
    T: Sync,
    F: FnMut(&T) -> Readiness,
{
    async fn is_ready(&mut self, tx: &T) -> Readiness {
        (*self)(tx)
    }
}

#[async_trait(?Send)]
impl<T, A, B> Ready<T> for (A, B)
where
    A: Ready<T>,
    B: Ready<T>,
    T: Sync,
{
    async fn is_ready(&mut self, tx: &T) -> Readiness {
        match self.0.is_ready(tx).await {
            Readiness::Ready => self.1.is_ready(tx).await,
            r => r,
        }
    }
}
