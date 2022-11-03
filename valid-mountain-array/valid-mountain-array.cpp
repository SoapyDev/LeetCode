class Solution {
public:

bool validMountainArray(vector<int>& arr) {

    int n = arr.size();

    if (n < 3){
        return false;
    }else if (arr[0] >= arr[1]){
        return false;
    } else if (arr[n-1] >= arr[n-2]){
        return false;
    }

    int i {1};

    while (arr[i] < arr[i+1] && i < n){
        i++;
    }
    
    for (i++; i < n ; ++i) {
        if (arr[i] >= arr[i-1]){
            return false;
        }
    }
    return true;
}

};