# Any copyright is dedicated to the Public Domain.
# https://creativecommons.org/publicdomain/zero/1.0/

import glob
import os

MPL_LICENSE_HEADER = '''/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
'''.encode()


assert __name__ == "__main__"

for manifest_path in glob.glob('**/Cargo.toml'):
    for src_path in glob.glob(os.path.dirname(manifest_path) + "/src/**", recursive=True):
        try:
            with open(src_path, 'r+b') as file:
                content = file.read()
                if not content.startswith(MPL_LICENSE_HEADER):
                    file.seek(0)
                    file.write(MPL_LICENSE_HEADER)
                    file.write(content)
        except IsADirectoryError:
            pass
