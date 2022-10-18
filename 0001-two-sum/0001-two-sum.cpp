class Solution {
public:
std::vector<int> twoSum(std::vector<int>& nums, int target) {
    std::vector<int> indexes {0,1};

    if(nums.size() == 2 || (nums[indexes[0]] + nums[indexes[1]]) == target){
        return indexes;
    }

    size_t j {0};

    for(size_t i = nums.size()-1; i > j; i--){

        for (; j < i; j++) {
            
          if (nums[i] + nums[j] == target){
                indexes[0] = i;
                indexes[1] = j;
          } else{
              continue;
          }
        }
           j=0;
    }
    return indexes;
}
};