class Solution {
public:
int removeElement(vector<int>& nums, int val) {

    int i=0;
    int n=nums.size();
    while(i!=n){
        if(nums[i]==val){
            nums.erase(nums.begin()+i);
            n=nums.size();
        }
        else{
            i++;
        }
    }
    return nums.size();
}
};