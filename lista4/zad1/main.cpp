#include <iostream>

#include "utils.hpp"
// Example prime numbers: 10631, 15299

int main(){
    int64_t a, b;
    int64_t p, q;
    int64_t dA, eA, dB, eB, n;
    std::pair<int64_t, int64_t> skA, pkA, skB, pkB;
    std::pair<int64_t, int64_t> private_result;

    std::cout << "Enter two positive prime numbers p and q:" << std::endl;
    std::cin >> a >> b;
    p = (int64_t) a;
    q = (int64_t) b;

    std::cout << a << " " << b << std::endl;
    std::cout << p << " " << q << std::endl;
    std::cout << "Checking if p and q prime" << std::endl;
    if (a < 0 || b < 0) {
        std::cout << "p and q must be positive!" << std::endl;
        return 0;
    } else if (!is_prime(p) || !is_prime(q)) {
        std::cout << "p and q must be prime!" << std::endl;
        return 0;
    } else {
        std::cout << "\t Confirmed" << std::endl;
    }

    n = p * q;

    std::cout << "Generating RSA for person A" << std::endl;
    generate_RSA(p, q, eA, dA);
    pkA.first = n;
    pkA.second = eA;
    skA.first = n;
    skA.second = dA;

    std::cout << "Generating RSA for person B" << std::endl;
    generate_RSA(p, q, eB, dB);
    pkB.first = n;
    pkB.second = eB;
    skB.first = n;
    skB.second = dB;

    private_result = calculate_private(n, pkA.second, skA.second);

    std::cout << "Calculated p: " << private_result.first << " Calculated q: " << private_result.second << std::endl;
}
