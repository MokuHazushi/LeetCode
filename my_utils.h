#ifndef MY_UTILS_H
#define MY_UTILS_H

#include <vector>
#include <string>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

ListNode* VectorToLinkedList(vector<int> &v);
vector<int> LinkedListToVector(ListNode *l);

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

string booleanToString(bool b);

template string NumberVectorToString<int>(vector<int> &v);

#endif