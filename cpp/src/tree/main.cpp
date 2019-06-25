#include "range_sum_query.hpp"

#include <assert.h>

class NumArray;

int main() {
  std::vector<int> nums = {1, 2, 3, 4, 5, 6, 7, 8};
  NumArray arr(nums);
  arr.printSegmentTree();
  std::cout << arr.sumRange(1, 6) << std::endl; // 3 + 5 + 7 + 8 + 9 + 10 = 42
  assert(arr.sumRange(1, 6) == 27);
  arr.update(1, 3);
  arr.printSegmentTree();
  assert(arr.sumRange(1, 6) == 28);


}
