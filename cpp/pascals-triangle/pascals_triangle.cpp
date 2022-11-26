#include "pascals_triangle.h"

namespace pascals_triangle {
    std::vector<std::vector<int>> generate_rows(int n){
        std::vector<std::vector<int>> ret {};
        //special case: Zero rows
        if(n==0) return ret;

        ret.push_back({1});
        for(int row=1; row<n; row++){
            std::vector<int> current_row {};

            for(int j=0; j<row+1; j++){
                if(j !=0 && j<row) current_row.push_back(ret[row-1][j-1] + ret[row-1][j]);
                else current_row.push_back(1);
            }

            ret.push_back(current_row);
        }
        return  ret;
    }
}  // namespace pascals_triangle
