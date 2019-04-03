#include "../src/RegularMatching.hpp"
#include "external/single_include/catch.hpp"


TEST_CASE("RegularMatching","[isMatch]") {
  REQUIRE(isMatch("123", "123") == true);
  //REQUIRE(isMatch("123", "*") == true); p[0] can't be '*'
  REQUIRE(isMatch("", "") == true);
  //REQUIRE(isMatch("abcd", "d*") == true);
  REQUIRE(isMatch("abcd", "d*") == false);
}
