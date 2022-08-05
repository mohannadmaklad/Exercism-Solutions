#include "triangle.h"
#include <stdexcept>

namespace triangle {
    flavor kind(double a, double b, double c)
    {
        if (a <= 0 || b <= 0 || c <= 0)
            throw std::domain_error("Invalid Input values");        
        if (a + b <= c || a + c <= b || b + c <= a)
            throw std::domain_error("Invalid Input values");        
        if (a == b && b == c)
            return flavor::equilateral;
        if((a == b && b != c) || (a == c && c != b) || (b == c && c != a))
            return flavor::isosceles;

        return flavor::scalene;
    }
}  // namespace triangle
