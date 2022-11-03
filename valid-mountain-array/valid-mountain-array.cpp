class Solution {
public:
bool validMountainArray(vector<int>& arr) {

    if (arr.size() < 3 || arr[0] > arr[1] || arr[arr.size()-1] > arr[arr.size()-2]){
        return false;
    }

    int pivot {0};
    while (arr[pivot] < arr[pivot+1] && pivot < arr.size()){
        pivot++;
    }

    for (; pivot < arr.size()-1 ; ++pivot) {
        if (arr[pivot] <= arr[pivot+1]){
            return false;
        }
    }

    return true;
}

};