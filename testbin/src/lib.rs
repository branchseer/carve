/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::io::stdout;

pub fn print_resources<'a>(resources: &[Option<&'static [u8]>]) {
    bincode::serialize_into(&mut stdout().lock(), resources).unwrap()
}
