import os
import re

def add_pub_to_struct_fields(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 简化后的正则表达式模式
    # 匹配结构体字段定义：可能前面有空格/注释/属性，然后是字段名和类型
    pattern = re.compile(
        r'((\s*//[^\n]*\n|\s*#\[[^]]*]|\s*)*)'  # 前面可能有注释或属性
        r'\s*(\w+)'                                # 字段名
        r'(\s*:\s*[^,\n]+([,\n]))'                 # 类型和结尾
    )

    def replacer(match):
        leading = match.group(1)  # 前面的注释/属性/空格
        field = match.group(3)    # 字段名
        rest = match.group(4)     # 类型和结尾

        # 如果已经有pub了就不添加
        if 'pub ' in leading:
            return match.group(0)
        return f"{leading}pub {field}{rest}"

    new_content = pattern.sub(replacer, content)

    if new_content != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Processed: {file_path}")
    else:
        print(f"No changes needed: {file_path}")

def process_directory(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                add_pub_to_struct_fields(file_path)

if __name__ == '__main__':
    models_dir = 'models'  # 修改为你的models目录路径
    if not os.path.exists(models_dir):
        print(f"Error: Directory '{models_dir}' does not exist")
    else:
        process_directory(models_dir)
        print("Processing completed!")