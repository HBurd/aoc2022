#include <iostream>
#include <fstream>
#include <cstdlib>
#include <cstdint>

int main()
{
    std::ifstream input_file("input.txt");

    char line[1024] = {};

    uint64_t max_calories = 0;
    uint64_t almost_max_calories = 0;
    uint64_t almost_almost_max_calories = 0;
    uint64_t current_calories = 0;
    uint64_t num_elves = 0;

    while (input_file.getline(line, sizeof(line)))
    {
        if (line[0])
        {
            char* end;
            current_calories += strtoull(line, &end, 10);
            if (current_calories > max_calories)
            {
                almost_almost_max_calories = almost_max_calories;
                almost_max_calories = max_calories;
                max_calories = current_calories;
            }
        }
        else
        {
            current_calories = 0;
            ++num_elves;
        }

    }

    std::cout << "Num elves: " << num_elves << "\n"
              << "Max calories: " << max_calories << "\n"
              << "Max 3: " << max_calories + almost_max_calories + almost_almost_max_calories << std::endl;

    return 0;
}
