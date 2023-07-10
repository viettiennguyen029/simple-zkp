# simple-zkp
Chaum-Pedersen Zero Knowledge Proof implements in Rust.

Steps:


```
0. In the Chaum-Pederson method, we initially define the values of <g, A, B, AB>
0. Victor and Peggy agree of (g,g^a, g^b and g^ab)

1. Peggy generates random number (r) - also known as secret
2. Peggy sends y1(g^r, B^r)

3. Victor sends a random value (c) challenge Peggy
4. Peggy computes z=r+a*s(mod q)

5. Victor now checks these are the same
5.1 Victor checks g^z
5.2 Victor checks A^c y1 (mod q)
```

For more information about Chaum-Pedersen Zero Knowledge Proof, visit: https://asecuritysite.com/encryption/chaum
