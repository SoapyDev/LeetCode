class Solution {
public:
std::vector<std::string> fizzBuzz(int n) {

    std::vector<std::string> answer{"1"};

    for (int i = 2; i <= n; ++i) {

        if (i%3==0 && i%5==0){
            answer.emplace_back("FizzBuzz");
        } else if(i%3==0){
            answer.emplace_back("Fizz");
        } else if(i%5==0){
            answer.emplace_back("Buzz");
        } else{
            answer.push_back(std::to_string(i));
        }
    }
    return answer;
}
};