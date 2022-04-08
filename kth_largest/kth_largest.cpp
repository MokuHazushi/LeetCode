#include <iostream>
#include <my_utils.h>
#include <vector>
#include <algorithm>

class KthLargest {
private:
    vector<int> heap;
    int kth;
public:
    KthLargest(int k, vector<int>& nums) {
        kth = k;
		heap = nums;

		if (heap.size() > k) {
			sort(heap.begin(), heap.end(), [](int i, int j){ return i>j; });
			heap.erase(heap.begin()+k, heap.end());
		}

        make_heap(
            heap.begin(),
			heap.end(), 
            [](int i, int j){ return i>j; });
    }
    
    int add(int val) {
		if (heap.size() < kth) {
			heap.push_back(val);
			push_heap(heap.begin(), heap.end(), [](int i, int j){ return i>j; });
			return heap.front();
		}

		if (val < heap.front())
			return heap.front();

        pop_heap(heap.begin(), heap.end(), [](int i, int j){ return i>j; });
		heap.pop_back();
		heap.push_back(val);
		push_heap(heap.begin(), heap.end(), [](int i, int j){ return i>j; });
        
        return heap.front();
    }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */

int main() {
	int k = 3;
	vector<int> nums({4,5,8,2});
	KthLargest *obj = new KthLargest(k, nums);
	vector<int> numToAdd({3,5,10,9,4});
	
	cout << "What is the " << k << "th largest element?" << endl;
	for (int n : numToAdd) {
		cout << "Adding " << n << " to the stream, largest is " << obj->add(n) << endl;
	}
	

	k = 1;
	nums = vector<int>();
	obj = new KthLargest(k, nums);
	numToAdd = vector<int>({-3, -2, -4, 0, 4});
	cout << "What is the " << k << "th largest element?" << endl;
	for (int n : numToAdd) {
		cout << "Adding " << n << " to the stream, largest is " << obj->add(n) << endl;
 	}

	k = 2;
	nums = vector<int>({0});
	obj = new KthLargest(k, nums);
	numToAdd = vector<int>({-1, 1, -2, -4, 3});
	cout << "What is the " << k << "th largest element?" << endl;
	for (int n : numToAdd) {
		cout << "Adding " << n << " to the stream, largest is " << obj->add(n) << endl;
 	}
}
