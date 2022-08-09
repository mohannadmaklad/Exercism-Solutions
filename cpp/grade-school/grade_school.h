#if !defined(GRADE_SCHOOL_H)
#define GRADE_SCHOOL_H

#include <vector>
#include <map>
#include <algorithm>  


namespace grade_school {
    class school {
        private:
            std::map<int, std::vector<std::string>> roster_;
            const std::vector<std::string> empty_;
        public:
            void add(const std::string &name, int grade)
            {
                roster_[grade].push_back(name);
                std::sort(roster_[grade].begin(), roster_[grade].end());
            }
            const std::map<int,std::vector<std::string>> roster() const
            {
                return roster_;
            }
            const std::vector<std::string>& grade(int grade) const
            {
                auto grade_it = roster_.find(grade);
                if(grade_it == roster_.end())
                {
                    return empty_;
                }
                return grade_it->second;
            }
    };

}  // namespace grade_school

#endif // GRADE_SCHOOL_H