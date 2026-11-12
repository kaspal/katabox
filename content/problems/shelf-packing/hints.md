Simple rules like "put each book on the first shelf where it fits" are fast but not always optimal. With at most 15 books, you can search: place the books one by one, trying every shelf for each.
---
Place the **thickest books first**: they are the hardest to fit, so wrong choices fail early. For each book, try every shelf already in use where it fits, and then one new shelf. Two shelves with the same load are interchangeable: trying one of them is enough.
---
Keep the best count found so far and stop a branch as soon as it uses as many shelves as the best. A good first best (from "first fit, thickest first") and the lower bound ⌈total / width⌉ (stop when reached) make the search fast.
