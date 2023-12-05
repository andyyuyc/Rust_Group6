#![allow(warnings)]

use file_management::commit::Commit;

use crate::{file_management::{commit::commit, hash::DVCSHash}, state_management::checkout::checkout};

pub mod file_management;
pub mod interface;
pub mod state_management;

pub fn main() {

}