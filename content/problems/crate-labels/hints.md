A label has no spaces, so splitting the whole input on whitespace gives `N` and then exactly one token per label. Each label then splits on `;` into entries, and each entry splits on `:` into a name and a quantity.
---
Keep a dictionary (hash map) from product name to its running total. For every entry, add its quantity to the name's total, starting from 0 the first time the name appears.
---
After all the labels, sort the dictionary's names with the plain string order of your language (for lowercase letters it is alphabetical, and `fig` comes before `figs`) and print `name total` for each, all in one write.
