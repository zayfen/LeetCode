#include "range_sum_query.hpp"

class NumArray;

int main() {
  std::vector<int> nums = {1, 3, 5};
  NumArray arr(nums);
  arr.printSegmentTree();
}
