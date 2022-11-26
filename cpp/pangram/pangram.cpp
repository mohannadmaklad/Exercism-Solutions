#include "pangram.h"
#include <bitset>

namespace pangram {
        bool is_pangram(std::string const &sentence)
        {
            std::bitset<26> flags;
            for(auto& c:sentence){
                if(std::isalpha(c)){
                    flags.set(std::tolower(c)-'a');
                }
            }
            return flags.all();
        }
}  // namespace pangram
