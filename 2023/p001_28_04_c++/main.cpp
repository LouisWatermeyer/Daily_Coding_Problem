// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.

// For examle, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

// Bonus: Can you do this in one pass?

#include <array>
#include <algorithm>
#include <iostream>

template <std::size_t SIZE>
bool checkIfAddableTo(const std::array<int, SIZE> sourceNumbers, const int targetNumber)
{
    for (int i = 0; i < sourceNumbers.size(); i++)
    {
        if (sourceNumbers[i] >= targetNumber)
            continue;

        for (int j = i + 1; j < sourceNumbers.size(); j++)
        {
            if (sourceNumbers[i] + sourceNumbers[j] == targetNumber)
                return true;
        }
    }

    return false;
}

template <std::size_t SIZE>
bool checkIfAddableToWithSort(const std::array<int, SIZE> sourceNumbers, const int targetNumber)
{
    // does the sorting count as a pass :D?
    std::array<int, SIZE> sortedNumbers = sourceNumbers;
    std::sort(sortedNumbers.begin(), sortedNumbers.end());

    int i = 0;
    int j = sortedNumbers.size() - 1;
    while (i < j)
    {
        if (sortedNumbers[i] + sortedNumbers[j] == targetNumber)
            return true;
        else if (sortedNumbers[i] + sortedNumbers[j] < targetNumber)
            i++;
        else
            j--;
    }
    return false;
}

template <std::size_t SIZE>
void printResult(const std::string algo, const std::array<int, SIZE> arr, const int k, bool result)
{
    printf("Using %s and the numbers: ", algo.c_str());
    for (int number : arr)
    {
        printf("%d, ", number);
    }
    printf("and k = %d, the result is %d\n", k, result);
}

int main()
{
    std::array<int, 4> arr = {10, 15, 3, 7};
    int k = 17;
    printResult("for-loop", arr, k, checkIfAddableTo(arr, k));
    printResult("sorting", arr, k, checkIfAddableToWithSort(arr, k));


    arr = {11, 5, 30, 72};
    k = 30;
    printResult("for-loop", arr, k, checkIfAddableTo(arr, k));
    printResult("sorting", arr, k, checkIfAddableToWithSort(arr, k));

    arr = {11, 5, 25, 72};
    k = 30;
    printResult("for-loop", arr, k, checkIfAddableTo(arr, k));
    printResult("sorting", arr, k, checkIfAddableToWithSort(arr, k));

    return 0;
}
