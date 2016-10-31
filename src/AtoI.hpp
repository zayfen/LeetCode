#ifndef ATOI_H
#define ATOI_H

#include <string>
#include <exception>
#include <limits.h>
#include <iostream>

using namespace std;

int atoi(const string &str) {
  int len = str.length();
  int s = 0;
  int e = len-1;
  while(s < len && isspace(str.at(s))) {
    s++;
  }
  while(e >= 0 && isspace(str.at(e))) {
    e--;
  }
  if (e == -1) {
    return 0;
  }
  string str2 = str.substr(s, e-s+1);
  len = str2.length();
  long long result = 0L;
  for(int i = len-1; i >= 0; i--) {
    char c = str2.at(i);
    if (c >= '0' && c <= '9') {
      result = result * 10 + (long long)(c-32);
    }
    if (i == 0) {
      if (c == '-') {
        result *= -1L;
      }
    }
  }
  if (result < INT_MIN) return INT_MIN;
  if (result > INT_MAX) return INT_MAX;
  return int(result);
}

#endif /* ATOI_H */
