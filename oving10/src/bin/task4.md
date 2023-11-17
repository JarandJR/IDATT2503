# Task 4
## a)
Show that encryption in RSA has the following property:
```bash
e_k(x_1) * e_k(x_2) mod n = e_k(x_1 * x_2) mod n
```

We define e_k as follows:
```bash
function e_k(x, e, n) {
    return x.pow(e) % n
}
```
### LHS
```bash
e_k(x_1) * e_k(x_2) mod n =
```
```bash
((x_1^e) mod n * (x_2^e) mod n) mod n =
```
```bash
((x_1^e) mod n * (x_2^e) mod n) mod n =
```
```bash
((x_1^e * x_2^e) mod n) mod n
```

### RHS
```bash
((x_1 * x_2)^e mod n) mod n =
```
```bash
((x_1^e * x_2^e) mod n) mod n
```

```bash
LSH = RSH
```

## b)
Show how RSA is vulnerable to chosen cipher text attack: For ciphertext y, then
Eva can choose some ```r ̸≡ 1 mod n```, and construct ```y′ = y · r^e```. If she then knows the decryption ```x′ = dK(y′)```, show how she can calculate ```x = dK(y)```. (Hint: She can also calculate ```r^−1 mod n```)

Given a public key ```e```, and a private key ```d```. Eva knows a ```r``` which is not congruent to ```n```. She knows a ciphertext ```y```, and calculates a new ciphertext.

```bash
y′ = y * r^e
```
```bash
𝑦′ ≡ 𝑦 ∗ 𝑟^e mod n
```
We can then express the decrypted ```x``` like this:
```bash
𝑥′ ≡ (𝑦′)^d 𝑚𝑜𝑑 n
```

From definition of ```y′``` we can reexpress this as:
```bash
𝑥′ ≡ (𝑦 ∗ 𝑟^𝑒)^d 𝑚𝑜𝑑 n
```
We know from following definition:
```bash
𝑑𝑒 ≡ 1(𝑚𝑜𝑑(𝑝 − 1)(𝑞 − 1)
```
That we have the following for ```r^ed```

```bash
𝑟^𝑒𝑑 ≡ 𝑟 𝑚𝑜𝑑 n
```
We therefore rexpress ```r^ed``` as ```r``` giving
```bash
𝑥′ ≡ (𝑦^𝑑 ∗ 𝑟) 𝑚𝑜𝑑 n
```
We know that Eva can calculate the inverse of r, and can therefore calculate the cleartext for x
with:
```bash
𝑥 ≡ 𝑥′ ∗ (𝑟^−1(𝑚𝑜𝑑 n))(𝑚𝑜𝑑 n)
```
