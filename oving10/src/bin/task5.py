def print_powers_of_number(n):
    for i in range(0, 101):
        print(n ** i)


def get_common_key(p, n, a, b):
    return (n**(a*b)) % p


if __name__ == "__main__":
    print("Task 5a:")
    print("Powers of 3:")
    print_powers_of_number(3)
    print("Powers of 5:")
    print_powers_of_number(5)
    print("\nThe difference is that the powers of 5 increment at a much faster rate than the powers of 3.")

    print("\nTask 5b:")
    print("Common key: ", get_common_key(p=101, n=3, a=33, b=65))
