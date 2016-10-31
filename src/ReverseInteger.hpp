#pragma once

#include <iostream>
#include <climits>

int reverseInteger(int x) {
  
  long long n = x < 0 ? -x : x;
  
  long long  result = 0;
  while (n > 0) {
    result = result * 10 + n%10;
    n = n / 10;
    
  }
  if (result < INT_MIN || result > INT_MAX) {
    return 0;
  }
  return x < 0 ? -result : result;
}

