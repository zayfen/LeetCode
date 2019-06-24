// 307. Range Sum Query - Mutable
// href: https://leetcode.com/problems/range-sum-query-mutable/

#include <iostream>
#include <vector>

class NumArray {
public:
  NumArray (std::vector<int>& nums): size(nums.size()) {
    this->segmentTree.resize(2 * size - 1); // 等比求和公式
    this->buildSegmentTree(nums);
  }

  void update (int i, int val) {
    return ;
  }

  int sumRange (int i, int j) {
    // 等比数列 q = 2, a0 = 1, Sn = 2*an - 1, beause an = nums.length, so Sn = 2 * nums.length - 1
    
    i = i + this->size - 1; // 指向树的最后一层
    j = j + this->size - 1; // 指向树的最后一层
    int sum = 0;

    while (i < j) {

      // 如果 i 或者 j是奇数，则 sum += segmentTree[i | j]
      if ((i & 0x1) == 0) { // i是偶数
        sum += this->segmentTree[i];
        std:: cout << "i: " << i << " ;sum: " << sum << std::endl;
        i++;
        
      }

      if ((j & 0x1) != 0) { // j是奇数
        sum += this->segmentTree[j];
        std:: cout << "j: " << j << " ;sum: " << sum << std::endl;
        j--;
      }
      i = (i - 1) / 2;
      j = (j - 2) / 2;
      std::cout << std::endl;
    }

    // if low < mid < high, 
    return sum;
  }

  void printSegmentTree () {
    for (auto i : segmentTree) {
      std::cout << i << " ";
    }
    std::cout << std::endl;
  }

private:

  void buildSegmentTree (std::vector<int>& nums) {
    int left = 0;
    int right = this->size - 1;

    int currentNodeIndex = 0;
    this->buildSegmentTreeHelper(left, right, nums, currentNodeIndex);
  }

  inline int mid (int left, int right) {
    return left + (right - left) / 2;
  }

  inline int leftChildIndex (int index) {
    return 2 * index + 1;
  }

  inline int rightChildIndex (int index) {
    return leftChildIndex(index) + 1;
  }

  /**
   ** 构建线段树
   ** @left: 当前节点的range left
   ** @right: 当前节点的range right
   ** @nums: 构建树的源数据
   ** @currentNodeIndex: 当前的节点的索引， 从0开始
   **/
  void buildSegmentTreeHelper (int left, int right, std::vector<int>& nums, int currentNodeIndex) {
    if (left >= right) {
      this->segmentTree[currentNodeIndex] = nums[left];
      return;
    }

    int middle = this->mid(left, right);

    // nums[curretNodeIndex] = nums[leftChildIndex] + nums[rightChildIndex]

    int leftChildIndex_ = this->leftChildIndex(currentNodeIndex);
    int rightChildIndex_ = this->rightChildIndex(currentNodeIndex);

    // calculate left child
    this->buildSegmentTreeHelper(left, middle, nums,  leftChildIndex_);
    this->buildSegmentTreeHelper(middle + 1, right, nums, rightChildIndex_);

    this->segmentTree[currentNodeIndex] = this->segmentTree[leftChildIndex_] + this->segmentTree[rightChildIndex_];
  }

private:
  std::vector<int> segmentTree;
  const int size = 0;
};
