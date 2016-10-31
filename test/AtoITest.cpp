#ifndef ATOITEST_H
#define ATOITEST_H

#include "../src/AtoI.hpp"
#include "external/single_include/catch.hpp"

TEST_CASE("AtoITest", "[myAtoi]") {
  string s("123");
  REQUIRE(myAtoi(s) == 123);
  string s1("1");
  REQUIRE(myAtoi(s1) == 1);
  string s_1("-1");
  REQUIRE(myAtoi(s_1) == -1);
  string s2("+1");
  REQUIRE(myAtoi(s2) == 1);
  string s3("-123");
  REQUIRE(myAtoi(s3) == -123);
  string s4("0");
  REQUIRE(myAtoi(s4) == 0);
  string s5("-00");
  REQUIRE(myAtoi(s5) == 0);
  string s6("00001");
  REQUIRE(myAtoi(s6) == 1);

  string s7("   123");
  REQUIRE(myAtoi(s7) == 123);
  string s8("2147483648");
  REQUIRE(myAtoi(s8) == 2147483647);
  string s9("+-2");
  REQUIRE(myAtoi(s9) == 0);
  string s10("   -0012a42");
  REQUIRE(myAtoi(s10) == -12);
  string s11("9223372036854775809");
  REQUIRE(myAtoi(s11) == 2147483647);
  string s12("-2147483648");
  REQUIRE(myAtoi(s12) == -2147483648);
}

#endif /* MYATOITEST_H */
