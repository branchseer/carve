/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "injector")]
pub mod injector;

#[cfg(feature = "injectee")]
#[doc(hidden)]
pub mod injectee;

#[doc(hidden)]
pub mod names;
