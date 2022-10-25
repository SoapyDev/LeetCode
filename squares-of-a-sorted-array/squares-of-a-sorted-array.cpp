class Solution {
public:
vector<int> sortedSquares(vector<int>& nums) {

    vector<int> numsList(nums.size());
    int low = 0;
    int high = nums.size()-1;
    int i = high;

    while(high >= low){
       if(abs(nums[high]) >= abs(nums[low])){
           numsList[i] = nums[high] * nums[high];
           high--;
       } else{
           numsList[i] = nums[low] * nums[low];
           low++;
       }
       i--;
    }
    return numsList;
}
};