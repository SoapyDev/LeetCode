class Solution {
public:
    int findNumbers(vector<int>& nums) {
        
        int evenNum {0};
        int tmp;
        int iteration {0};
        
        for(auto &num: nums){
            tmp = num;
            iteration = 0;
            
            while(tmp > 0){
                tmp/=10;
                ++iteration;
            }
            
            if(iteration%2 == 0){
                evenNum++;
            }
            
        }  
        return evenNum;
    }
};