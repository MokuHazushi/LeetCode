#ifndef MY_UTILS_H
#define MY_UTILS_H

#include <vector>
#include <map>
#include <string>

using namespace std;

/* STRUCTURES */

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

/* TEMPLATE METHODS */

template<typename T>
string NumberVectorToString(vector<T> &v) {

    if (v.empty())
        return string("[]");

    string s("[");

    if (v.size() == 1) {
        s.push_back(v[0]);
        s.push_back(']');
        return s;
    }

    for (int i=0; i<v.size()-1; i++) {
        s.append(to_string(v[i]));
        s.append(", ");
    }
    s.append(to_string(v[v.size()-1]));
    s.push_back(']');
    return s;
}
template string NumberVectorToString<int>(vector<int> &v);
template string NumberVectorToString<bool>(vector<bool> &v);

template<typename T>
string DoubleEntryArrayToString(T **tab, int size) {
    if (size == 0)
        return string("[]");
    
    string s("[\n");
    for (int i=0; i<size; i++) {
        vector<T> tmp;
        s.append("\t");

        for (int j=size-1; j>=0; j--) {
            tmp.push_back(tab[i][j]);
        }
        
        s.append(NumberVectorToString(tmp));
        s.append("\n");
    }
    s.append("]");
    return s;
}
template string DoubleEntryArrayToString(bool **tab, int size);

template<typename T>
string NumberMapToString(map<T, T> &m) {
    string s;

    if (m.empty())
        return string("Empty map");

    for (auto it=m.begin(); it!=m.end(); ++it) {
        s.append(to_string(it->first));
        s.append(" -> ");
        s.append(to_string(it->second));
        s.push_back('\n');
    }
    return s;
}
template string NumberMapToString(map<int, int> &m);


/* METHODS */
ListNode* VectorToLinkedList(vector<int> &v);
vector<int> LinkedListToVector(ListNode *l);
string booleanToString(bool b);



#endif