#include <bits/stdc++.h>

const int T_S = 200;

class HashTableEntry {
    public:
        int k;
        int v;
        HashTableEntry(int k, int v){
            this->k = k;
            this->v = v;
        }
};

class HashMapTable {
    private:
        HashTableEntry **t;
    public: 
        HashMapTable(){
            t = new HashTableEntry * [T_S];
            for (int i = 0; i< T_S; i++){
                t[i] = NULL;
            }
        }
        int HashFunc(int k){
            return k % T_S;
        }
        void Insert(int k, int v){
            int h = HashFunc(k); //bucket number
            while (t[h] != NULL && t[h]->k != k){ //if h is occupied and the current key in the occupied cell is not k
                h = HashFunc(h + 1); //apply hash func to h+1
            }

            //t[h]->k == k
            if (t[h] != NULL)
                delete t[h];
            
            //now t[h] == NULL for sure
            t[h] = new HashTableEntry(k, v);
        }
        int SearchKey(int k){
            int h = HashFunc(k);
            while (t[h] != NULL && t[h]->k != k){
                h = HashFunc(h+1);
            }
            if (t[h] == NULL)
                return -1;
            else 
                return t[h] -> v;
        }
        void Remove(int k){
            int h = HashFunc(k);
            while (t[h] != NULL){
                if (t[h]->k == k)
                    break;
                h = HashFunc(h + 1);
            }
            if (t[h] == NULL){
                std::cout << "No element found at key" << k << std::endl;
                return;
            }
            else {
                delete t[h];
            }
            std::cout << "Element Deleted" << std::endl;
        }
        ~HasMapTable(){
            for (int i=0; i < T_S; i++){
                if (t[i] != NULL)
                    delete t[i];
                delete[] t;
            }
        }
};