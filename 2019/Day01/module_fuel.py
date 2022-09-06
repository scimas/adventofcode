import os


def get_module_fuel():
    base_dir = os.getcwd()
    data_dir = os.path.join(base_dir, "data")
    total_fuel = 0

    with open(os.path.join(data_dir, "Day01_input.txt"), "r") as infile:
        text = infile.readline().strip()
        while text:
            module_mass = float(text)
            module_fuel = module_mass // 3 - 2
            total_fuel += module_fuel
            text = infile.readline().strip()
    
    return total_fuel


if __name__ == "__main__":
    print(get_module_fuel())
