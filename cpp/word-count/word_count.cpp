#include "word_count.h"
#include <vector>
#include <map>
#include <algorithm>

namespace word_count
{
    /* Remove ignored stuff */
	void clean_string(std::string& s)
	{
		std::vector<char> to_ignore = {'\n', '&' , '@', '$', '%', '^','&','.' , ':' , '\"' , '!', ','}; 
		auto is_apostrophe = [](const std::string& s,const size_t& i){return s[i-1] != ' ' &&  s[i+1] != ' ' && s[i+1] != '\0';};
		//transform to lower case
		std::transform(s.begin(), s.end(), s.begin(), [](char c){return std::tolower(c);});
		//remove single quotes
		for(size_t i=0; i<s.size(); i++){
			if(s[i] == '\'' && !is_apostrophe(s,i)) s[i] = ' ';
		}
		//removed ignored
		for(auto& c:s){
			if(std::find(to_ignore.begin(), to_ignore.end(),c) != to_ignore.end()) c = ' ';

		}
	}

	std::vector<std::string> split(std::string s){
		int start_ind{}, end_ind{};
		bool valid_word = false;
		std::vector<std::string> res;

		for(const auto& c : s){		
			if(c == ' '){
				if(valid_word) res.push_back(s.substr(start_ind,end_ind-start_ind));
				start_ind = end_ind +1;
				valid_word = false;
			} else{
				valid_word = true;
			}
			++end_ind;
		}

		if(valid_word){
			res.push_back(s.substr(start_ind,end_ind-start_ind));
		}

		return res;
	}

	std::map<std::string, int> words(std::string st)
	{
		clean_string(st);
		auto sp = split(st); 
		std::map<std::string,int> count;

		for(const auto& res:sp){
			++count[res];
		}

		return count;
	}
}  // namespace word_count