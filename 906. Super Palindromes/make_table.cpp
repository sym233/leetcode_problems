#include<iostream>
bool isParlindrome(long long a) {
    long long t = a;
    long long r = 0;
    while (t != 0) {
        int i = t % 10;
        r *= 10;
        r += i;
        t /= 10;
    }
    return r == a;
}
// this program doesn't take a long time
// for n = 1e9, only tens of seconds

int main(void) {
    int n = 1e9;
    for (long long i = 1; i < n; i++) {
        if (isParlindrome(i) && isParlindrome(i * i)) {
            std::cout << (i * i) << ", ";
        }
    }
    std::cout << "done" << std::endl;
    return 0;
}
