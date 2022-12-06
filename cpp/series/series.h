#if !defined(SERIES_H)
#define SERIES_H

#include <vector>
#include <string>

namespace series {
    const std::vector<std::string> slice(const std::string& s, int length);
}  // namespace series

#endif // SERIES_H