#include "hamming.h"
#include <stdexcept>
#include <numeric>

namespace hamming {
    unsigned int compute(const std::string& a,const std::string& b)
    {
        if(a.length() != b.length())
            throw std::domain_error("Strings are not of equal length");
        
        return std::inner_product(a.begin(),a.end(),b.begin(),0,std::plus<int>(), std::not_equal_to<int>());
    }
}  // namespace hamming
