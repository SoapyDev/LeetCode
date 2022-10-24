class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        
        int max = 0;
        int tmp = 0;
        
        for(auto &num: nums){
            if(num == 1){
                ++tmp;
                if(tmp > max){
                    max = tmp;
                }
            }else{
                tmp = 0;
            }
        } 
        
        return max;
    }
};