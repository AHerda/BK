#ifndef UTILS_HPP
#define UTILS_HPP

#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>
#include <numeric>
#include <random>
#include <cstdint>

int64_t gcd_extended(int64_t a, int64_t b, int64_t& x, int64_t& y);
int64_t modular_inverse(int64_t e, int64_t k);
bool is_prime(int64_t number);
void generate_RSA(int64_t p, int64_t q, int64_t &e, int64_t &d);
int64_t mod_pow(int64_t base, int64_t exp, int64_t mod);
std::pair<int64_t, int64_t> calculate_private(int64_t n, int64_t e, int64_t d);

#endif
