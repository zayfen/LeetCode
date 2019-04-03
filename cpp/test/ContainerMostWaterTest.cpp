#include "../src/ContainMostWater.hpp"
#include "external/single_include/catch.hpp"

TEST_CASE("ContainMostWater", "[getMostWater]") {
  std::vector<int> v(3, 0);
  v.push_back(1);
  v.push_back(3);
  v.push_back(2);
  REQUIRE(getMostWater(v) == 2);
}
