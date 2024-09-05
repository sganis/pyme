#!/usr/bin/env python3

import os
import json
import re
import subprocess

def run(cmd):
    subprocess.run(cmd, shell=True, check=True)

def update_file(path, version):
    with open(path) as r:
        lines = r.readlines()
    with open(path, 'wt') as w:
        for line in lines:
            m = re.match(r'(^\s*"?version"?\s*)([:=]\s*)"(.+)"(.*)', line)
            if m:
                newline = f'{m.group(1)}{m.group(2)}"{version}"{m.group(4)}\n'
                w.write(newline)
                print(f'updated version: {version} in {path}')
            else:
                w.write(line)

with open('./version.txt') as r:
    version = r.read().strip()
assert version

update_file('./frontend/package.json', version)
update_file('./Cargo.toml', version)


print(f'realeasing v{version}...')
run('npm --prefix ./frontend run build')
run('git add ./frontend')
run(f'git commit -am "v{version}"')
run(f'git tag -a v{version} -m v{version}')
run('git push --follow-tags')
# run('cargo shuttle deploy')
print('done.')

