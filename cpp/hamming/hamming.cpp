#include "hamming.h"
#include <stdexcept>

namespace hamming {
    unsigned int compute(const std::string& a,const std::string& b)
    {
        if(a.length() != b.length())
        {
            throw std::domain_error("Strings are not of equal length");
        }

        unsigned int distance = 0;
        for(unsigned int i = 0; i< a.length(); i++)
        {
            if(a[i] != b[i])
            {
                distance++;
            }
        }
        return distance;
    }
}  // namespace hamming
