# Obviously correct: try adding 0, 1, 2, … letters in front; the letters added must be the
# word's last k letters reversed, so try each k and keep the first palindrome.
def main() -> None:
    s = input().strip()
    for k in range(len(s) + 1):
        candidate = s[len(s) - k :][::-1] + s
        if candidate == candidate[::-1]:
            print(candidate)
            return


main()
