import os

LINKER_SCRIPT = 'src/linker.ld'
APPS = os.listdir('src/bin')
APPS.sort()

BASE_ADDR = 0x80400000
SIZE_LIMIT = 0x20000

CYAN = '\033[96m'
YELLOW = '\033[93m'
RESET = '\033[0m'

def read_from_file(file_path):
    with open(file_path, 'r') as file:
        return file.read()

def write_to_file(file_path, content):
    with open(file_path, 'w') as file:
        file.write(content)

def replace_content(file_path, old_content, new_content):
    content = read_from_file(file_path).replace(old_content, new_content)
    write_to_file(file_path, content)

def main():
    with open(LINKER_SCRIPT, 'r') as file:
        raw = file.read()
    for app in APPS:
        addr = BASE_ADDR + SIZE_LIMIT * APPS.index(app)
        replace_content(LINKER_SCRIPT, hex(BASE_ADDR), hex(addr))
        print(f'Building {CYAN}{app}{RESET} at {YELLOW}{hex(addr)}{RESET}')
        os.system(f'cargo build --release --bin {app.split('.')[0]}')
        write_to_file(LINKER_SCRIPT, raw)

if __name__ == '__main__':
    main()