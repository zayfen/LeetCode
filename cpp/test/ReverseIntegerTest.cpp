#include "../src/ReverseInteger.hpp"

#include "external/single_include/catch.hpp"


TEST_CASE("ReverseInteger", "[reverseInteger]") {
  REQUIRE(reverseInteger(0) == 0);
  REQUIRE(reverseInteger(-1) == -1);
  REQUIRE(reverseInteger(1) == 1);
  REQUIRE(reverseInteger(10) == 1);
  REQUIRE(reverseInteger(123) == 321);
  REQUIRE(reverseInteger(-123) == -321);
  REQUIRE(reverseInteger(1534236469) ==  0);
}
