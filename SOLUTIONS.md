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

## Day 8
### Part 1 
For the first part, the algorithm is a basic map traversal. I first parsed the input in to a list of instructions and a map of `{current node: (left move, right move)}` just like the input is formatted. This lets us find the next steps to take in constant time, so solving the problem is just a matter of looking up the current position in the hashmap, setting the current position to either the left or right move depending on the current instruction, and repeat this until we land on `ZZZ` while keeping a counter of the number of iterations we take. 

To handle the parsing, I chose to implement a regex this time for fun since I don't practice regex a lot and this problem seemed like an easy one for using regex to save a few lines of code. I would probably prefer just using `split` in production code for such a simple parsing operation though, since that is way more readable than even the simple regex I wrote. 

To handle the situation of repeating over the instructions in order until we reached the target node, I implemented my logic to treat the list of steps like a queue. On each loop I dequeue the instruction and then add it back to the end of the queue and just keep doing that until we're done with the traversal. Less memory efficient than using iterators but didn't seem to have a big imapct on time for solving this part.

### Part 2
My first attempt was to brute force this by modifying my code from part 1 to increment the positions of a list of starting positions rather than a single starting position. This predictably doesn't scale and after running it for a few minutes on my laptop, I realized that there is probably some math involved. 

For starters, I refactored my part 1 code to generalize the looping function that counts the steps from an arbitrary starting point until an arbitrary ending condition is met. The ending condition parameter is a function so that I could reuse the function for both part 1 and 2 without much copy/pasting of code, since part 1 the function just needs to check if the current position is `ZZZ` but part 2 we have to check if the position simply ends in `Z`. 

I made the (luckily correct) assumption that each starting point would make a cycle by the time it lands on a valid ending point, such that if it takes 10 steps to get to an ending point, and we continue the instructions we were given, we won't hit another valid ending point until exactly 10 steps again. 

If this is true ( which is is for the given problem input), the answer to how many steps it takes before every starting point simultaneously is on a valid ending point is the Least Common Multiple(LCM) of the number of steps it takes for each starting point to reach an ending point. So the algorithm is roughly 

- parse the input to instructions and a map of points and valid directions to move
- collect a list of all the points starting with the letter `A`
- for each starting point `P`, run the calculation from part 1 to find the total steps it takes to reach a valid ending point 
- find the least common multiple of all the step counts


 A little research found that a fast way to find the LCM of two numbers a and b is `lcm(a,b) = a * b / hcf(a,b)`. Finding the highest common factor (HCF) of two numbers quickly is to just check the factors of the smaller number (in descending order) to see if the larger number is evenly divisibible by the same factor. This approach minimizes the amount of time to factorize a number, so even though we end up finding the HCF beween some numbers in the billions, we never need to factorize a number greater than 21,000. 

 ## Day 9 
 ### Part 1 
 Nothing particularly tricky about this. Just implemented the algorithm exactly as described, keeping a list of `Vec`s representing the differences at each stage of calculating the extrapolated values and then working backwards once I find an iteration that is all `0`s. 

 ### Part 2 
 Same here, the only difference for extrapolating backwards is looking at the first element in the `Vec` of diffs at each stage instead of the last one, and making sure to subtract the values between each step in the calculation for the extrapolation rather than adding. 

 ## Day 10 
 ### Part 1
 For this problem we can take several assumptions given to us about the input to come up with a strategy
 1. there is a valid starting point 'S'
 2. there are exactly two valid connecting pipes to each cell in the grid that is part of the pipe 
 3. the 'pipe' makes a closed loop, so if we start two paths traversing the opposite directions away from the starting point, they will eventually collide. 

Since we need to find the farthest point, the approach here is to do a breadth-first-search of the pipe until we hit a collision, and then find the point that has the farthest distance recorded. In addition to the standard BFS algorithm, we have to check at each point in the paths only for valid adjacent pipes to make sure we're actually on the right pipe path. To do this, given that we know which character corresponds to what type of pipe, we can do the following 
1. for current cell `c`, look up the possible valid adjacent cells to `c` 
2. get the values of the valid adjacent cells to our current cell, refer to them as `a`
3. for every `a`, look up valid adjacent cells for `a` to see if `c` is a valid adjacent cell to `a`

this ensures we're only traversing paths for cells that are properly connected pipes. To get the valid adjacent cells, I just hardcoded lists of deltas for valid adjacent cells based on the pipe type (vertical, left bend, right bend, etc). Once we have a couple helper functions to find the valid adjacent cells to a given cell, the rest of the implementation of part 1 is a standard BFS 

### Part 2
My initial approach was to perform a depth-first search (DFS) from every coordinate to the boundary of the grid to find which points did not have a valid path to the end of the grid. This approach basically follows the following steps
1. collect list of all pipe points in the path, same as part 1 just we don't care about distances from the start.
2. take every point in the grid that is not in the pipe path and DFS to the edge of the grid 
   1. valid DFS moves are any direction as long as the move doesn't intersect the pipe path 
3. count the points which have no valid route to the edge of the grid. 

This mostly worked except for edge cases where some points were prevented from having a valid path to the edge of the grid but also were not enclosed in the main loop. This comes up when there's a very windy pipe path. 

After researching and consulting peers working on this, I learned about the approach of ray casting. Basically, this means you iterate over every point, "cast a ray" infinitely right to the end of the maze, and count the number of intersections of this ray with the pipe path. Then take the minimum value of the intersection of north or south bound intersections and if the number is odd, that means the point is enclosed in the loop. 

"casting a ray" basically means build a list of points extending in a direction (here, it's horizontally right) from the starting point to the end of the grid boundary. So in a 5x5 grid, the ray from piont `(1,1)`  to the end of the grid is `[(1,1), (1,2), (1,3),(1,4)]`. This approach properly accounts for weird pipe shapes and lets us get the answer. 
