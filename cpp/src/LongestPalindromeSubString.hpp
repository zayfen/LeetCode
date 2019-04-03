//

#pragma once

#include <string>
#include <stack>
 
class LongestPalindromeSubString
{
public:
  LongestPalindromeSubString(){}
  virtual ~LongestPalindromeSubString(){}
  const std::string getLongestPalindromeSubString(const std::string& str) {
    std::string result("");
    if (str.empty()) {
      return "";
    }
    result = str.at(0);
    const int len = str.length();
    std::stack<int> table[300];
    for (int i = 0; i < len; i++) {
      table[size_t(str.at(i))].push(i);
    }

    int maxflag = 0;
    for (int i = 0; i < len; i++) {
      std::stack<int> v = table[size_t(str.at(i))];
      while (v.size() > 0 && v.top() > i) {
        int j0 = i;
        int j1 = v.top();
        if (j1 <= maxflag) {
          break;
        }
        
        while (j0 < j1) {
          if (str.at(j0) == str.at(j1)) {
            j0++;
            j1--;
          } else {
            break;
          }
        }
        if (j0 >= j1) {
                
          size_t size = v.top() - i + 1;
          if (size > result.size()) {
            result = str.substr(i, size);
            maxflag = v.top();
          }
          break;
        }
        v.pop();
      }
    }
    return result;
  }
};
