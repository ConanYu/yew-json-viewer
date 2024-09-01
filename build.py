import os
import json
import shutil
import sys
import subprocess

def main():
    # build json parser
    cwd = os.path.dirname(os.path.abspath(__file__))
    process = subprocess.run('trunk build --release --dist docs --public-url="/yew-json-viewer"', shell=True, 
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
    sys.stdout.write(process.stdout.decode('utf-8'))
    sys.stderr.write(process.stderr.decode('utf-8'))
    process.check_returncode()

    # build library
    main_rs_path = os.path.join(cwd, 'src', 'bin', 'main.rs')
    with open(main_rs_path, 'r', encoding='utf-8') as f:
        source = f.read()
    with open(main_rs_path, 'w', encoding='utf-8') as f:
        f.write('''#[allow(unused_imports)]
use yew_json_viewer::{json_view_render, JsonViewRenderOption};
fn main() {}''')
    process = subprocess.run('trunk build --release --dist build', shell=True, 
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
    with open(main_rs_path, 'w', encoding='utf-8') as f:
        f.write(source)
    with open(os.path.join(cwd, 'build', 'package.json'), 'w', encoding='utf8') as f:
        f.write(json.dumps({
            "name": "yew-json-viewer",
            "version": "1.0.3"
        }, ensure_ascii=False, indent=2))
    sys.stdout.write(process.stdout.decode('utf-8'))
    sys.stderr.write(process.stderr.decode('utf-8'))
    process.check_returncode()

    with os.scandir(os.path.join(cwd, 'build')) as entries:
        for entry in entries:
            if entry.is_file() and entry.name.endswith('.wasm'):
                os.rename(entry.path, os.path.join(cwd, 'build', 'main_bg.wasm'))
            elif entry.is_file() and entry.name.endswith('.js'):
                os.rename(entry.path, os.path.join(cwd, 'build', 'main.js'))

    process = subprocess.run('wasm-pack build', shell=True,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
    sys.stdout.write(process.stdout.decode('utf-8'))
    sys.stderr.write(process.stderr.decode('utf-8'))
    process.check_returncode()
    with open(os.path.join(cwd, 'build', 'main.d.ts'), 'w', encoding='utf8') as f:
        f.write('export default function init(): Promise<void>;\n\n')
        with open(os.path.join(cwd, 'pkg', 'yew_json_viewer.d.ts'), 'r', encoding='utf8') as dts:
            f.write(dts.read())

    shutil.rmtree(os.path.join(cwd, 'pkg'))
    os.remove(os.path.join(cwd, 'build', 'index.html'))


if __name__ == '__main__':
    main()
