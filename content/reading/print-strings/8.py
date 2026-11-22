text = "the quick brown fox"
words = text.split(" ")
print(words[1][::-1])
print(" ".join(w[0] for w in words))
