#ifndef UTILS_HPP
#define UTILS_HPP

#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>
#include <numeric>
#include <random>
#include <cstdint>

uint64_t gcd_extended(uint64_t a, uint64_t b, uint64_t& x, uint64_t& y);
uint64_t modular_inverse(uint64_t e, uint64_t k);
bool is_prime(uint64_t number);
void generate_RSA(uint64_t p, uint64_t q, uint64_t &e, uint64_t &d);
uint64_t mod_pow(uint64_t base, uint64_t exp, uint64_t mod);
std::pair<uint64_t, uint64_t> calculate_private(uint64_t n, uint64_t e, uint64_t d);

#endif
