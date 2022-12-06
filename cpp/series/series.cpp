#include "series.h"
#include <stdexcept>


namespace series {
    const std::vector<std::string> slice(const std::string& s, int length){
        
        if(length <= 0 || (int)s.length() < length) 
            throw std::domain_error("Slice range is out of boundaries");
        
        std::vector<std::string> slices;
        for(size_t i=0; i<s.length()-length+1; i++){
            slices.push_back(s.substr(i,length));
        }
        
        return slices;
    }
}  // namespace series
