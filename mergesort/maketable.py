import numpy as np

data_filename = "4/data.csv"
table_filename = "table.md"

def main():
    for i in range(6):
        data_filename = f"4/data{i}.csv"
        table_filename = f"4/table{i}.md"
        with open(data_filename, "r") as file:
            lines = file.readlines()
        
        data = np.array([[int(x) for x in line.split(",")] for line in lines])
        
        with open(table_filename, "w") as file:
            file.write("| Input Size | Processing Time (ms) | Number of Comparisons |\n")
            file.write("| ---------- | -------------------- | --------------------- |\n")
            for size, time, num_c in data:
                file.write(f"| {size} | {time} | {num_c}\n")

if __name__ == "__main__":
    main()