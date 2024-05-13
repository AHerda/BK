#include "utils.hpp"

std::random_device rd;
std::mt19937 gen(rd());

int64_t gcd_extended(int64_t a, int64_t b, int64_t& x, int64_t& y) {
    if (a == 0) {
        x = 0;
        y = 1;
        return b;
    }

    int64_t x1, y1;
    int64_t gcd = gcd_extended(b % a, a, x1, y1);

    x = y1 - (b / a) * x1;
    y = x1;

    return gcd;
}

int64_t modular_inverse(int64_t e, int64_t k) {
    int64_t x, y;
    int64_t gcd = gcd_extended(e, k, x, y);

    if (gcd != 1) {
        // Modular inverse does not exist
        return -1;
    } else {
        // Ensure d is positive
        int64_t d = (x % k + k) % k;
        return d;
    }
}

bool is_prime(int64_t number){
    if (number <= 1 || number % 2 == 0 || number % 3 == 0)
        return false;
    if (number <= 3)
        return true;
    for(int64_t i = 5; i * i <= number; i += 6)
        if(number % i == 0 || number % (i + 2) == 0)
            return false;

    return true;
}

void generate_RSA(int64_t p, int64_t q, int64_t &e, int64_t &d) {
    std::vector<int64_t> possible_e;
    int64_t random_index;
    int64_t phi = (p - 1) * (q - 1);

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

int64_t mod_pow(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
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

std::pair<int64_t, int64_t> calculate_private(int64_t n, int64_t e, int64_t d){
    int64_t phi = d * e - 1;
    int64_t t = phi;
    int64_t a = 2;
    int64_t k, x, p, q;
    std::pair<int64_t, int64_t> result;

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
