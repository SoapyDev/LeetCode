class Solution {
public:
    int findNumbers(vector<int>& nums) {
        
        int evenNum {0};
        int tmp;
        int iteration {0};
        
        for(auto &num: nums){
            iteration = floor(log10(num))+1;
                
            if(iteration%2 == 0){
                evenNum++;
            }
            
        }  
        return evenNum;
    }
};