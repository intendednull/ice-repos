use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use serde::Deserialize;

use yewdux::prelude::*;

#[derive(Clone, Eq, PartialEq, Deserialize, Debug)]
pub struct Repository {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    pub updated_at: DateTime<Local>,
    pub pushed_at: DateTime<Local>,

    // #[serde(flatten)]
    // extras: HashMap<String, Value>,
}

pub struct DesiredArchiveState {
    pub id: usize,
    pub desired_archive_state: bool
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct Organization {
    pub name: Option<String>
}

/// The desired state for a given repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveState {
    /// We have chosen in the pagination view to _not_ archive this repository.
    Skip,
    /// We have chosen in the pagination view to archive this repository.
    Archive,
    /// We have changed from "to archive" to "don't archive" in the review view.
    SkippedInReview
}

impl ArchiveState {
    /// Convert a boolean, essentially the toggle state of a checkbox in the
    /// Paginator component and convert it into an ArchiveState. In the
    /// paginator, we want to use the Skip state to indicate that we do not
    /// want to see this archive in the review phase.
    pub fn from_paginator_state(b: bool) -> Self {
        match b {
            true => Self::Archive,
            false => Self::Skip
        }
    }

    /// Convert a boolean, essentially the toggle state of a checkbox in the
    /// Review & Submit component and convert it into an ArchiveState. In
    /// the review, we want to use the SkippedInReview to indicate that we
    /// do want to continue to see this archive in the review phase.
    pub fn from_review_state(b: bool) -> Self {
        match b {
            true => Self::Archive,
            false => Self::SkippedInReview
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct ArchiveStateMap {
    // Map from the repository ID as a key, to a pair
    // containing the Repository struct and a boolean
    // indicating whether we want to archive that repository
    // or not.
    pub map: BTreeMap<usize, (Repository, ArchiveState)>
}

impl ArchiveStateMap {
    pub fn with_repos(&mut self, repositories: &[Repository]) -> &mut Self {
        for repo in repositories {
            if !repo.archived {
                self.map.entry(repo.id).or_insert((repo.clone(), ArchiveState::Archive));
            }
        }
        self
    }

    #[must_use]
    pub fn get_desired_state(&self, id: usize) -> Option<bool> {
        self.map
            .get(&id)
            .map(|(_, archive_state)| match archive_state {
                ArchiveState::Archive => true,
                _ => false
            })
    }

    pub fn update_desired_state(&mut self, id: usize, desired_archive_state: ArchiveState) -> &mut Self {
        web_sys::console::log_1(&format!("Updating {id} to {desired_archive_state:?}").into());
        self.map.entry(id).and_modify(|p| { p.1 = desired_archive_state });
        web_sys::console::log_1(&format!("The resulting map was {self:?}").into());
        self
    }

    pub fn get_repos_to_review(&self) -> impl Iterator<Item = &Repository> {
        self.map
            .values()
            .filter_map(|(repo, to_archive)| {
                (*to_archive != ArchiveState::Skip).then_some(repo)
            })
    }

    pub fn get_owned_repos_to_review(&self) -> Vec<Repository> {
        self.get_repos_to_review().cloned().collect()
    }
}
