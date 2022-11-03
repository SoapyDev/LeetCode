class Solution {
public:
bool checkIfExist(vector<int>& arr) {

    int cur {};
    int j {};
    int size = arr.size();
    for (int i = 0; i < size; i++){
        cur = arr[i] * 2;
        for (j = 0; j < size; j++){
            if (arr[j] == cur && j != i){
                return true;
            }
        }
    }
    return false;
}
};