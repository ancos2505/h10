use std::{ops::Deref, sync::RwLock};

use crate::{ROOT_PAGER_COUNTER, server::ServerResult};

#[derive(Debug, Default)]
struct PageCounter {
    inner: RwLock<Clicks>,
}

#[derive(Debug, Default)]
struct Clicks(usize);

impl Deref for Clicks {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Clicks {
    pub(crate) fn inc(&mut self, n: usize) {
        self.0 += n
    }
}

impl PageCounter {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn add_one(&mut self) -> ServerResult<()> {
        let mut w = self.inner.write()?;
        w.inc(1);
        Ok(())
    }
    pub(crate) fn read(&self) -> ServerResult<usize> {
        Ok(**self.inner.read()?)
    }
}
