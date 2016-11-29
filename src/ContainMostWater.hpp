/**
 *   \file ContainMostWater.hpp
 *   \brief A Documented file.
 *
 *  Detailed description
 *
 */


#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

/**
 *  \brief function description
 *
 *  Detailed description
 *
 *  \param param
 *  \return return type
 */
int getMostWater(std::vector<int> vec) {
  int result = 0;
  const int len = vec.size();
  int left = 0;
  int right = len-1;
  while (left < right) {
    int leftVal = vec.at(left);
    int rightVal = vec.at(right);
    result = std::max(result, (right-left) * std::min(leftVal, rightVal));
    if (leftVal < rightVal) {
      left++;
      continue;
    }
    right --;
  }

  return result;
}



