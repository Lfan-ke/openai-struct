"""
刚刚add_pub将所有use也pub了，现在需要remove use pub的pub...
"""

import os
import re

def remove_pub_from_use(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 匹配 "use pub" 并移除 "pub"（保留可能的空格格式）
    pattern = re.compile(r'(\buse\s+)pub(\s+.+)')
    new_content = pattern.sub(r'\1\2', content)

    if new_content != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Fixed 'use pub' in: {file_path}")
    else:
        print(f"No 'use pub' found in: {file_path}")

def process_directory(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                remove_pub_from_use(file_path)

if __name__ == '__main__':
    target_dir = 'models'
    if not os.path.exists(target_dir):
        print(f"Error: Directory '{target_dir}' does not exist")
    else:
        process_directory(target_dir)
        print("Finished removing 'pub' from 'use' statements!")