/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! APIs for injecting or listing resources in executables

mod inject;
mod list;

pub use inject::inject;
pub use list::list;
