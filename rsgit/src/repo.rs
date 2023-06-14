use crate::{storage::Storage, worktree::WorkTree};

// Repository a git repo
pub struct Repository<'a> {
    store: &'a dyn Storage,
    wt: &'a WorkTree,
}

impl<'a> Repository<'a> {
    pub fn new(store: &'a dyn Storage, wt: &'a WorkTree) -> Self {
        Repository {
            store: store,
            wt: wt,
        }
    }
}
