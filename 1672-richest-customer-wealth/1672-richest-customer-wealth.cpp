class Solution {
public:
int maximumWealth(std::vector<std::vector<int>>& accounts) {

    int max {};
    int tmp {};

    for (const auto& customer: accounts) {
        for (auto& bank: customer) {
            tmp += bank;
        }
        if (tmp > max){
            max = tmp;
        }
        tmp = 0;
    }

    return max;
}
};