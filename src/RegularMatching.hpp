/**
 *   \file RegularMatching.hpp
 *   \brief A Documented file.
 *
 *  Detailed description
 *
 */

#ifndef REGULARMATCHING_H
#define REGULARMATCHING_H

#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <iomanip>

using namespace std;

std::nullptr_t t = nullptr;

/**
 *  \brief bool isMatch(string s, string p)
 *
 *
 *
 *  \param param
 *  \return return type
 */

void printDpFun(vector<vector<bool> > dp_f) {
  const int i = dp_f.size();
  const int j = dp_f[0].size();
  for (int i_ = 0; i_ < i; i_++) {
    std::cout << "\n";
    for (int j_ = 0; j_ < j; j_++) {
      cout << std::left  << setw(3) << dp_f[i_][j_];
    }
  }
}


bool isMatch(const string s, const string p) {
  const int s_len = s.size();
  const int p_len = p.size();

  vector<vector<bool>> dp_f(s_len + 1, vector<bool>(p_len + 1, false));
  // init dp_f
  dp_f[0][0] = true;

  // assume p is empty
  for (size_t i = 1; i <= s_len; i++) {
    dp_f[i][0] = false;
  }

  // assume that s is empty
  for (size_t j = 1; j <= p_len; j++) {
    dp_f[0][j] = j > 1 && p[j - 1] == '*' && dp_f[0][j-2];
  }

  for (size_t i = 1; i <= s_len; i++) {
    for (size_t j = 1; j <= p_len; j++) {
      if (p[j - 1] == '*') {
        dp_f[i][j] = dp_f[i][j-2] || (s[i-1] == p[j-2] || '.' == p[j-2]) && dp_f[i-1][j];
      } else {
        dp_f[i][j] = dp_f[i-1][j-1] && (s[i-1] == p[j-1] || '.' == p[j-1]);
      }
    }
  }

  std::cout << "s: " << s << "  p: " << p << std::endl;
  printDpFun(dp_f);
  return dp_f[s_len][p_len];
}

#endif /* REGULARMATCHING_H */
