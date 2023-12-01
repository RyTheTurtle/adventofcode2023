# Solutions 

## Day 1 
### Part 1 
For this approach, I simply iterate over the input lines one by one, recording the first and last number I encountered as I iterate over each char in each input line. Although it would have been faster to have an iterator on the front and back of the strings to more quickly find the first and last digit in the average case, the worst case would be the same as the approach of just using a single iterator. 

Solution scales linearly with the number of input lines and size of each line.

### Part 2 
The naive approach I took first was to simply do pre-processing to find/replace instances of 
digit spellings with their numeric representations. This failed certain cases where the spellings
of adjacent digits shared a last letter. For example, `eightwothree` becomes either `8wo3` or `eigh23` depending on whether you first replace `eight` or if you first replace `two`. 

The next approach I took was to do a more proper parsing, iterating across the string from front to back and, at each alphabetic character, test to see if there was a sequence of characters that matched one of the spellings of a digit and do the replacement then. This worked for situations like `eightwothree` because it always ensured we got the correct first digits, but suffered the same issue of overlapping spellings when finding the last digit in the string. As an example, `ninesevensrzxkzpmgz8kcjxsbdftwoner` was getting translated to `97srzxkzpmgz8kcjxsbdf2ner`, incorrectly using `2` as the last digit of the string instead of `1`. 

The simple fix to this was to first find the first digit from the beginning, and then find the first digit from the end of the string by checking progressively larger substrings from the end of the string. In the above example, the working backwards approach checks the following substrings in this order 

```
r -> None 
er -> None 
ner -> None 
oner -> 1
```

This approach worked correctly. Time complexity remains the same, scaling linearly with the size of the input. 

## Day 2 
### Part 1 
This problem is pretty straightforward string parsing. To implement part 1, I just iterate over the input lines, parse the text in to a struct representing a game and the rounds played. Once we have the game structs with the rounds, I filtered the list of games by which games have all the rounds be within bounds of the maximum cube counts, and then summed the values of the IDs of those games. The parsing itself wasn't very complex so I achieved that mostly by using `string.split` on various separators to extract the relevant information. Excluding underlying costs of `split`, this approach scales linearly with the size of input 

### Part 2
Since part 1 already took the time to parse the input text in to structs representing games and rounds, part 2 simply reused the parsing logic and computed the minimum number of each cube needed for each game to be valid by taking the maximum value for each cube color from all the rounds of that game. Then it's a simple multiplication and summation of the minimum cube counts for each round across the games to get the answer. Like part 1, this also scales linearly with the size of the input. 

## Day 3
### Part 1 
This took me an insane amount of time to get correct due to parsing the input properly. The approach is pretty straightforward to find the symbols in the grid by iterating and checking for any char that isn't a number or a `.`. Once you have the grid coordinates where the symbols are, you just check the adjacent cells to see if  any of them have a number. The tricky part is that if you find an adjacent cell with a digit, you have to 
 
1. scan left and right of the digit you found to see if it's part of a multi-digit number 
2. verify the multi-digit number wasn't already associated to another symbol

This took me several hours for some reason to figure out a good parsing strategy, but ultimately just used a few `HashSet`s for deduping to get the job done. 

### Part 2
This is relatively easy now that I figured out the parsing from part 1. The main changes to the logic are going to be 

1. instead of checking any non-digit, non `.` as a symbol, look explicitly for `*`. 
2. when scanning gear symbols for adjacent numbers, I'm going to explicitly count how many adjacent numbers are present to make sure it's `2`. If not, I can ignore the numbers and the symbol entirely since the problem calls for `exactly` two adjacent numbers
3. I don't think I need to dedup all coordinates across the input, just for a single gear. This narrows the scope of the deduping work

## Day 4
### Part 1 
This part is pretty straightforward to implement using some basic string splitting, `HashSet`s to dedup and do fast lookups on strings, and keep a running sum. Since the points of each card doubles as it has more winning numbers, the calculation for how much a card is worth can be done by either `2 ^ x` or by doing a left shift on `1` every time we encounter a new winning number, but for simplicity sake we'll do simple bit shifting.

### Part 2
Part 2 doesn't change the parsing but does change the logic about how to collect and score cards. It might not be the most efficient approach, but my initial approach is to store a separate `vec` of card counts represented as integers that get updated as I score them. 

The simple `O(n^2)` algorithm is to 
- preprocess the list of cards to convert each card to an integer of how many winning numbers are present on the card
- initalize a list (or `vec` in rust) of card counts for each card, defaulting every card count to `1`
- iterate over each card's winning numbers count, incrementing the next cards in the list by the total number of the current card. 

As an example, for the sample test case given in the problem, the iterations of the card counts look like this 
```
Iteration 0: [1, 1, 1, 1, 1, 1]
Iteration 1: [1, 2, 2, 2, 2, 1]
Iteration 2: [1, 2, 4, 4, 2, 1]
Iteration 3: [1, 2, 4, 8, 6, 1]
Iteration 4: [1, 2, 4, 8, 14, 1]
Iteration 5: [1, 2, 4, 8, 14, 1]
Result: 30
```

Might not be the most efficient algorithm but it works for the problem input. My suspicion is that the problem would be exponentially more difficult if someone tries to solve by actually adding new card instances to the list of cards as they "win" copies. 