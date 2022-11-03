class Solution {
public:
int removeElement(vector<int>& nums, int val) {

    int j = nums.size()- std::count(nums.begin(), nums.end(),val);
    std::remove(nums.begin(), nums.end(),val);
    
    return j;
}
};