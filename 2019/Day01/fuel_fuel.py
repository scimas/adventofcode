import os

def get_fuel_fuel():
    base_dir = os.getcwd()
    data_dir = os.path.join(base_dir, "data")
    modules_fuel = []

    with open(os.path.join(data_dir, "Day01_input.txt"), "r") as infile:
        text = infile.readline().strip()
        while text:
            module_mass = float(text)
            module_fuel = module_mass // 3 - 2
            extra_fuel = module_fuel // 3 - 2
            while extra_fuel > 0:
                module_fuel += extra_fuel
                extra_fuel = extra_fuel // 3 - 2
            modules_fuel.append(module_fuel)
            text = infile.readline().strip()
    
    total_fuel = sum(modules_fuel)
    return total_fuel


if __name__ == "__main__":
    print(get_fuel_fuel())
