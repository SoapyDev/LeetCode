class Solution {
public:
bool checkIfExist(vector<int>& arr) {

    int cur {};
    for (int i = 0; i < arr.size(); i++){
        cur = arr[i] * 2;
        for (int j = 0; j < arr.size(); j++){
            if (arr[j] == cur && j != i){
                return true;
            }
        }
    }
    return false;
}
};