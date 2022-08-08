#if !defined(GRADE_SCHOOL_H)
#define GRADE_SCHOOL_H

#include <string>
#include <vector>
#include <map>
#include <algorithm>  


namespace grade_school {
    class school {
        private:
            std::map<int, std::vector<std::string>> internal_roster;
        public:
            void add(const std::string &name, int grade)
            {
                internal_roster[grade].push_back(name);
                 std::sort(internal_roster[grade].begin(), internal_roster[grade].end());
            }
            const std::map<int,std::vector<std::string>> roster() const
            {
                return internal_roster;
            }
            std::vector<std::string> grade(int grade) const
            {
                auto grade_it = internal_roster.find(grade);
                if(grade_it == internal_roster.end())
                {
                    return std::vector<std::string>();
                }
                std::vector<std::string> ret = grade_it->second;
                return ret;
            }
    };

}  // namespace grade_school

#endif // GRADE_SCHOOL_H