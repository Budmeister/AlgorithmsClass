import numpy as np

folder = "4/"
data_filename = folder + "data.csv"
table_filename = "table.md"

def main():
    with open(data_filename, "r") as file:
        lines = file.readlines()
    
    data = np.array([[int(x) for x in line.split(",")] for line in lines])

    datas = np.split(data, 6)
    for i, data in enumerate(datas):
        with open(f"{folder}data{i}.csv", "w") as file:
            for datapoint in data:
                datapoint = [str(x) for x in datapoint]
                file.write(",".join(datapoint) + "\n")
    

if __name__ == "__main__":
    main()