// 307. Range Sum Query - Mutable
// href: https://leetcode.com/problems/range-sum-query-mutable/

#include <iostream>
#include <vector>

class NumArray {
public:
  NumArray (std::vector<int>& nums): size(nums.size()) {
    this->segmentTree.resize(2 * size); // 等比求和公式
    this->buildSegmentTree(nums);
   }

  void update (int i, int val) {
    for (this->segmentTree[i += size] = val; i > 1; i >>= 1) {
      segmentTree[i >> 1] = segmentTree[i] + segmentTree[i^1];
    }
    return ;
  }

  int sumRange (int i, int j) {
    int sum = 0;
    for (i += size, j+= size; i < j; i >>= 1, j >>= 1) {
      if (i & 1) { // 奇数
        sum += segmentTree[i++];
      }
      if (j & 1) { // 奇数
        sum += segmentTree[--j];
      }
    }
    return sum;
  }

private:

  void buildSegmentTree (std::vector<int>& nums) {
    for (int i = this->size; i < 2 * this->size; i++) {
      this->segmentTree[i] = nums[i - this->size];
    }

    for (int i = this->size - 1; i > 0; i--) {
      // this->segmentTree[i] = this->segmentTree[this->size * 2] + this->segmentTree[this->size * 2 + 1];
      // there is an more effective method
      this->segmentTree[i] = this->segmentTree[i << 1] + this->segmentTree[i << 1 | 1];
    }
  }

private:
  std::vector<int> segmentTree;
  const int size = 0;
};
