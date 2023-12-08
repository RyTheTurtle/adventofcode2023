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

## Day 5
### Part 1
The problem input with ranges looks like it's going to be prohibitively expensive to try to create a large array that holds every possible number, so I'm taking the approach of simply storing the ranges of each map and implementing functions to find the corresponding value for a given input using some math. 

For a given source number `s`, we can check if it fits in the range for one of the map entries by checking if `s >= src range start && s < src range start + range`. If it is in one of the ranges, we can find the corresponding value for `s` by calculating offset `s - src range start` and then finding `destination range start + offset` to get the value. This should end up being `O(number of maps * number of ranges in each map)`.  

### Part 2
The main difference, besides a bit of parsing, is that we now have a lot more seeds to inspect. I was originally going to brute force this, but that proved to be impractical given the large ranges. The next obvious solution seems to be to check what seed ranges overlap with each other.

First, I'll attempt to dedup the seed ranges in the almanac so there's no overlapping ranges. Then, I'll do some math on the ranges of seeds compared to the starting ranges of the first map. Range r1 overlaps with range r2 if 
```
r1.start >= r2.start && r1.start < r2.end 

```
I was going to continue researching algorithms for efficient range querying, but I was running the brute force approach in parallel and it happened to finish (correctly) before I could get a better solution coded so stopping for today. Took ~30 minutes to compute part 2 by brute force. 

By brute force on my M2 Pro macbook, here's the output stats from the problem input 
```
took 1352s to evaluate range
total seeds to test : 1917300386
Evaluated answer in 431s
Result: <censored>
```
Most of the time was accumulating a set of 1.9B "seeds" to run through the same logic as part 1. I got the correct answer, but we can optimize this to not take 30+ minutes to run. 

Checking every seed meant evaluating the logic from part 1 for roughly 1.9B iterations. To optimize this with a fairly straightforward approach, we can our approach to operate on ranges rather than on individual numbers. The `intersection`  of a range of numbers is the numbers that overlap between both ranges. The `difference` is what numbers are unique to each range. Computing these results as ranges, for any given input range being looked up in a map of ranges, I should at most get 3 ranges as outputs: The intersection, the lower end of the difference range, the upper end of the difference range. This will still make the solution `O(n)` where `n` is the number of ranges, but it won't scale linearly with the magnitude or the size of each range. Then it is just a matter of doing some math on the ranges to find all the "ouput ranges" from one of the seed almanacs, and then repeating that process for each of the ranges until we get to a list of ranges of location numbers that we can min.


## Day 6 
### Part 1 and 2
Since the only difference is the input size, combining these descriptions. There wasn't really any trick to this day other than to use a math operation to compute whether or not a particular number of ms pressing the button at the start of the race would out-perform the current record distance. 

## Day 7 
### Part 1 
Solving part 1 is pretty straightforward after finishing parsing the input. Anticipating we'll need to rank hands differently in part 2, I decided to take some time to build out separate structs and enums for cards, hands, and bids, including creating a bid comparison function to use. 

### Part 2
Brute forcing part 2 by evaluating all permutations of each hand will dramatically increase the scope of the inputs we have to evaluate, making it not feasible. Instead, we know that hands have 5 cards and depending on each type of hand, changing the jokers to a different card will have a more optimal effect and these can be hard coded to make the added logic effectively constant (the overall solution is still the cost of sorting bids). 

For example, if we have a hand that has a 4 of a kind, we can always create a 5 of a kind. If we have a hand that is a 5 of a kind, that's already the maximum hand. 

If we have a hand that is a full house, it depends on how many jokers we have:
- 0 jokers, max rank possible is 3 of a kind 
- 2 or 3 jokers, max rank possible is 5 of a kid 

if we have a 3 of a kind, depends on how many jokers 
- 0 jokers, max rank is 3 of a kind
- 1 joker, max rank is 4 of a kind 
- no other possibilities, because if we had a 3 of a kind and 2 jokers, it'd be a full house 

do this evaluation for all the hands and just hard code the logic. 

The function signatures are a bit weird, but I parameterized the functions used for parsing the hand and ranking the hands so that I could swap out the strategy for how I rank the hands (using jokers logic or not) to minimize the duplication between parts 1 and 2.