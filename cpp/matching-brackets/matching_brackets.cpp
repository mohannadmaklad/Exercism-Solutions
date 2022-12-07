#include "matching_brackets.h"
#include <stack>

namespace matching_brackets {

    bool check(std::string input){
        std::stack<char> brackets;
        auto is_open_bracket  = [](const char& c){return c == '(' || c == '[' || c == '{';};
        auto is_close_bracket = [](const char& c){return c == ')' || c == ']' || c == '}';};
        auto match_bracket = [](const char& b){return b == '('? ')': b =='['? ']' : b == '{'? '}':'-';};
        auto pop_stack = [](std::stack<char>& s) {char ret = s.top();s.pop();return ret;};

        for(const char& c : input){
            if(is_open_bracket(c)) brackets.push(c);
            else if(is_close_bracket(c) && (brackets.empty() || match_bracket(pop_stack(brackets)) != c))
                return false;
        }

        return brackets.empty()? true:false;
    }

}  // namespace matching_brackets
