# Euclidean & Extended Euclidean Algorithm

Euclidean algorithm is a way of finding the greatest common divisor between two positive integers.

The Extended Euclidean algorithm also finds the greatest common divisor between two positive numbers as well as the coefficents `s` and `t` of Bezout's identity:

```
a*x + b*y = gcd(a,b) 
```
It is one of the main tools for calculating the modular multiplicative inverses when `a` and `b` are coprime i.e it simplifies to:

```
ax + by = 1

where: ax = 1 (mod b), by = 1 (mod a) 
````

where x is the modular multplicative inverse of a mod b and y is the modular multplicative inverse of b mod a   

## Note

Used this crate for handling euclid div and remainders: https://github.com/BartMassey/euclid-divmod/blob/master/src/lib.rs  

Paper on euclidean definitions of div and remainder implementation is worth a read: https://dl.acm.org/doi/pdf/10.1145/128861.128862  