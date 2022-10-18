class Solution {
public:
std::vector<std::string> fizzBuzz(int n) {

    if(n == 1){
        std::vector<std::string> answer{"1"};
        return answer;
    } else if(n == 2){
        std::vector<std::string> answer{"1"};
        answer.emplace_back("2");
        return answer;
    } else{
        std::vector<std::string> answer{};
    }
    std::vector<std::string> answer{"1"};
    answer.emplace_back("2");
    for (int i = 3; i <= n; ++i) {

        if (i%3==0 && i%5==0){
            answer.emplace_back("FizzBuzz");
        } else if(i%3==0){
            answer.emplace_back("Fizz");
        } else if(i%5==0){
            answer.emplace_back("Buzz");
        } else{
            answer.emplace_back(std::to_string(i));
        }
    }
    return answer;
}
};