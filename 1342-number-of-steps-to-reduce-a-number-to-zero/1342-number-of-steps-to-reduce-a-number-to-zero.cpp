class Solution {
public:
int numberOfSteps(int num) {
    int tmp = 0;
    while (num != 0) {
        if (num %2 == 0){
            num /= 2;
            ++tmp;
        } else{
            num--;
            ++tmp;
        }
    }
    
    return tmp;
}
};