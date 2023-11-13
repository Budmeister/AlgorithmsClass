from random import randint
from tqdm import tqdm

with open("animal_names_corrected.txt", "r") as file:
    animals = [x.strip() for x in file.readlines()]


for N in tqdm(range(100_000, 1_000_001, 100_000)):
    lines = []
    seen = {animal: 0 for animal in animals}
    for _ in range(N):
        animal = animals[randint(0, len(animals) - 1)]
        seen[animal] += 1
        animal += f" {seen[animal]}"

        taz = randint(0, 5_000_000)
        tav = randint(0, 5_000_000)
        lines.append(f"{taz} {tav} {animal}\n")

    with open(f"output/test_input_{N}.txt", "w") as file:
        file.write(f"{N}\n")
        file.writelines(lines)