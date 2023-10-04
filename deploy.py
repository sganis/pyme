import os
import json
import subprocess

def run(cmd):
    subprocess.run(cmd, shell=True, check=True)

with open('./frontend/package.json') as r:
    js = json.loads(r.read())

version = js['version']
assert version

print(f'deploying v{version}...')
run('npm --prefix ./frontend run build')
run('git add ./frontend')
run(f'git commit -am "v{version}"')
run(f'git tag -a v{version} -m v{version}')
run('git push --follow-tags')
# run('cargo shuttle deploy')
print('done.')

