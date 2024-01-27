/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
mod bindgen;

#[cfg(feature = "injector")]
extern crate link_cplusplus;

#[cfg(feature = "injectee")]
pub mod postjectee;

#[cfg(feature = "injector")]
pub mod postjector;
