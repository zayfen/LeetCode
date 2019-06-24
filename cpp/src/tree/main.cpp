#include "range_sum_query.hpp"

class NumArray;

int main() {
  std::vector<int> nums = {1, 3, 5, 7, 8, 9, 10, 11};
  NumArray arr(nums);
  arr.printSegmentTree();
  std::cout << arr.sumRange(1, 6) << std::endl; // 3 + 5 + 7 + 8 + 9 + 10 = 42
}
