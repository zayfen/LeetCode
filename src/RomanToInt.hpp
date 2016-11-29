/**
 *   \file RomanToInt.hpp
 *   \brief A Documented file.
 *
 *  Detailed description
 *number range: 1-3999
 */


#include <iostream>
#include <algorithm>
#include <map>
#include <string>
#include <vector>

int romanToInt(const std::string &roman) {
  std::map<char, int> map = {{'M', 1000}, {'D', 500}, {'C', 100}, {'L', 50},
                             {'X', 10},   {'V', 5},   {'I', 1}};
  std::vector<int> vec;
  std::string roman_ = roman;
  std::for_each(roman_.begin(), roman_.end(), [&](char c) {
      int n = map[c];
    if (vec.empty()) {
      vec.push_back(n);
      return;
    }
    if (n < vec.back()) {
      n = -1 * n;
    }
    vec.push_back(n);
  });

  int res = 0;
  std::for_each(vec.begin(), vec.end(), [&res](int n){
      res += n;
                                        });
  return res;
}

int main(int argc, char *argv[])
{
  std::string roman = "XXXIX";
  std::cout << romanToInt(roman);
  return 0;
}

