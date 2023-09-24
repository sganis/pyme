# deploy to vercel with git
# using cli: vercel --prod
import os
import json
import subprocess

def run(cmd):
    subprocess.run(cmd, shell=True, check=True)

with open('./client/package.json') as r:
    js = json.loads(r.read())

version = js['version']
assert version

print(f'deploying v{version}...')
run('call npm --prefix ./client run build')
run('git add ./client')
run(f'git commit -am "v{version}"')
run(f'git tag -a v{version} -m v{version}')
run('git push --follow-tags')
print('done.')

