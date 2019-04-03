#pragma once

/*

**/
#include <cmath>
#include <iostream>


bool isPalindromeInteger(int x) {
  if (x < 0) {
    return false;
  }
  int size = -1;
  int x_ = x;
  do {
    size ++;
    x_ /= 10;
  } while (x_ != 0);

  x_ = x;
  int headNum = x_ / pow(10, size);
  int tailNum = x_ % 10;
  
 while (x_ != 0) {
   std::cout << "[H : T] => [" << headNum << " : " << tailNum << "]\n";
   if (headNum != tailNum) {
     std::cout << "!!!!!!!!NOT\n";
     return false;
   }
   x_ -= headNum * pow(10, size);
   x_ /= 10;
   size -= 2;

   headNum = x_ / pow(10, size);
   tailNum = x_ % 10;
 }
 std::cout << "**** YES \n";

 return true;
}
