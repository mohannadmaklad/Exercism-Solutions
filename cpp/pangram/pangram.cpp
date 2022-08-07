#include "pangram.h"

#define NUMBER_OF_LETTERS 26


namespace pangram {
        bool is_pangram(std::string const &sentence)
        {
            const char * alphabet = "abcdefghijklmnopqrstuvwxyz";
            for(auto i = 0; i < NUMBER_OF_LETTERS; i++)
            {
                if(sentence.find(alphabet[i]) == std::string::npos && sentence.find(std::toupper(alphabet[i])) == std::string::npos)
                {
                    return false;
                }
            }
            return true;
        }
}  // namespace pangram
