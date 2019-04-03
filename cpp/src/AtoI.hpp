#ifndef ATOI_H
#define ATOI_H

#include <algorithm>
#include <exception>
#include <iostream>
#include <limits.h>
#include <string>

using namespace std;

int myAtoi(string &str) {
  int len = str.length();
  int s = 0;
  int e = len - 1;
  while (s < len && isspace(str.at(s))) {
    s++;
  }
  while (e >= 0 && isspace(str.at(e))) {
    e--;
  }
  if (e == -1) {
    return 0;
  }

  string str2 = str.substr(s, e - s + 1);

  len = str2.length();
  long long result = 0L;
  int sign = 1;
  int index = 0;
  if (str2.at(0) == '-') {
    sign = -1;
    index = 1;
  }
  if (str2.at(0) == '+') {
    index = 1;
  }
  for (int i = index; i < len; i++) {
    char c = str2.at(i);
    if (c >= '0' && c <= '9') {
      result = result * 10L + (c - '0');
      if (result*sign <= (long long)INT_MIN) // NOTE: here is result(*sign not only result)
        return INT_MIN;
      if (result*sign >= (long long)INT_MAX)
        return INT_MAX;
    } else {
      break;
    }
  }
  result *= sign;
  return int(result);
}

#endif /* ATOI_H */
