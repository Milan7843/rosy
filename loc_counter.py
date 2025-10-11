import os

def count_lines(filename):
    with open(filename, 'r') as file:
        lines = file.readlines()
        # Remove empty lines and lines containing only whitespace
        #lines = [line for line in lines if line.strip()]
        return len(lines)

def count_lines_in_directory(directory, excluded_files=None):
    lines_dict = {}
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(('.rs')):
                file_path = os.path.join(root, file)
                if excluded_files and file_path.split('\\')[-1] in excluded_files:
                    continue
                lines = count_lines(file_path)
                lines_dict[file_path] = lines

    sorted_lines = sorted(lines_dict.items(), key=lambda x: x[1], reverse=True)
    for file, lines in sorted_lines:
        print(f"{file}: {lines} lines")

    total_lines = sum(lines_dict.values())
    print(f"Total lines: {total_lines}")

# Set the directory path where the files are located
directory_path = 'tests/'

# Set the list of excluded files
excluded_files = []

# Call the function to count lines in the directory
count_lines_in_directory(directory_path, excluded_files)