#include "../src/IsPalindromeInteger.hpp"
#include "external/single_include/catch.hpp"

TEST_CASE("IsPalindromeInteger", "[isPalindromeInteger]") {
  REQUIRE(isPalindromeInteger(11) == true);
  REQUIRE(isPalindromeInteger(1) == true);
  REQUIRE(isPalindromeInteger(12321) == true);
  REQUIRE(isPalindromeInteger(131) == true);
  REQUIRE(isPalindromeInteger(-11) == false);
  REQUIRE(isPalindromeInteger(12) == false);
  REQUIRE(isPalindromeInteger(-2147447412) == false);
}
