#ifndef ATOITEST_H
#define ATOITEST_H

#include "../src/AtoI.hpp"
#include "external/single_include/catch.hpp"


TEST_CASE("AtoITest", "[atoi]") {
  REQUIRE(atoi("123") == 123);
  REQUIRE(atoi("1") == 1);
  REQUIRE(atoi("-1") == -1);
  REQUIRE(atoi("+1") == 1);
  REQUIRE(atoi("-123") == -123);
  REQUIRE(atoi("0") == 0);
  REQUIRE(atoi("-00") == 0);
  REQUIRE(atoi("00001") == 1);
  REQUIRE(atoi("  123  ") == 123);
  REQUIRE(atoi("99999999999") > INT_MAX);
}

#endif /* ATOITEST_H */
