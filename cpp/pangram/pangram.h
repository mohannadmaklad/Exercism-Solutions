#if !defined(PANGRAM_H)
#define PANGRAM_H

#include <string>

namespace pangram {
    bool is_pangram(std::string const &sentence);
}  // namespace pangram

#endif // PANGRAM_H