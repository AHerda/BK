#include "utils.hpp"

std::random_device rd;
std::mt19937 gen(rd());

uint64_t gcd_extended(uint64_t a, uint64_t b, uint64_t& x, uint64_t& y) {
    if (a == 0) {
        x = 0;
        y = 1;
        return b;
    }

    uint64_t x1, y1;
    uint64_t gcd = gcd_extended(b % a, a, x1, y1);

    x = y1 - (b / a) * x1;
    y = x1;

    return gcd;
}

uint64_t modular_inverse(uint64_t e, uint64_t k) {
    uint64_t x, y;
    uint64_t gcd = gcd_extended(e, k, x, y);

    if (gcd != 1) {
        // Modular inverse does not exist
        return -1;
    } else {
        // Ensure d is positive
        uint64_t d = (x % k + k) % k;
        return d;
    }
}

bool is_prime(uint64_t number){
    if (number <= 1 || number % 2 == 0 || number % 3 == 0)
        return false;
    if (number <= 3)
        return true;
    for(uint64_t i = 5; i * i <= number; i += 6)
        if(number % i == 0 || number % (i + 2) == 0)
            return false;

    return true;
}

void generate_RSA(uint64_t p, uint64_t q, uint64_t &e, uint64_t &d) {
    std::vector<uint64_t> possible_e;
    uint64_t random_index;
    uint64_t phi = (p - 1) * (q - 1);

    std::cout << "\tCalculating e" << std::endl;
    // Wybór liczby e, która jest względnie pierwsza z phi
    for (e = 2; e < phi; e++) {
        if (std::gcd(e, phi) == 1){
            possible_e.push_back(e);
        }
    }

    std::cout << "\tSetting random e" << std::endl;
    std::uniform_int_distribution<> dis(0, possible_e.size() - 1);
    random_index = dis(gen);

    e = possible_e[random_index];

    std::cout << "\tCalculating d" << std::endl;
    d = modular_inverse(e, phi);
}

uint64_t mod_pow(uint64_t base, uint64_t exp, uint64_t mod) {
    uint64_t result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) {
            result = (result * base) % mod;
        }
        base = (base * base) % mod;
        exp >>= 1;
    }
    return result;
}

std::pair<uint64_t, uint64_t> calculate_private(uint64_t n, uint64_t e, uint64_t d){
    uint64_t phi = d * e - 1;
    uint64_t t = phi;
    int a = 2;
    uint64_t k, x, p, q;
    std::pair<int, int> result;

    while(t % 2 == 0){
        t = std::floor(t / 2);
    }

    // Szukamy a i k takich że (a^k)^2 = 1 mod n
    while(a < 100){
        k = t;
        while(k < phi){
            x = mod_pow(a, k, n);

            if(x != 1 && x != (n-1) && mod_pow(x, 2, n) == 1){
                p = std::gcd(x-1, n);
                break;
            }
            k *= 2;
        }
        a += 2;
    }
    q = n / p;

    result.first = p;
    result.second = q;

    return result;
}
