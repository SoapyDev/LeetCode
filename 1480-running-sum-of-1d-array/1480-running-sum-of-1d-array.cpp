class Solution {
public:

std::vector<int> runningSum(std::vector<int>& nums) {


    std::vector<int> sums {0};

    for (int & num : nums) {
        sums.push_back(num + sums.back());
    }

    sums.erase(sums.begin());
    return sums;
}
};