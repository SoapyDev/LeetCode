class Solution {
public:
bool canConstruct(std::string ransomNote, std::string magazine) {

    std::map<char, int> letters;
    std::map<char,int>::iterator it;


    for (auto& letter: ransomNote) {
        it = letters.find(letter);
        if(it->first == letter){
            it->second--;
        } else{
            letters[letter] = -1;

        }
    }

    for (auto& letter: magazine) {
        it = letters.find(letter);
        if(it->first == letter){
            it->second++;
        } else{
            letters[letter] = 1;
        }
    }

    for (auto& letterFr: letters) {
        if(letterFr.second < 0){
            return false;
        }
    }
    return true;
}
};