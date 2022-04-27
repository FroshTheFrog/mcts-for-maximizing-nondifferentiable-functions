# Overview
This project served two functions. The first was that I wanted to learn the programing language Rust. The second was to experiment with using MCTS to procedurally generate content. This would work by replacing instances where one would traditionally use evolutionary algorithms. You can see how it would work given the following example. Let’s say you wanted to construct an optimal leg for movement. In MCTS each action that could be taken would be a mutation to the leg. Given that the algorithm is not running to the end of a game, the play out strategy would only run to a set depth. Rather than backpropagating if the agent won or not, the max output of a utility function run on each state encountered along the path is returned. In the case of a leg example, the utility functions would output how effective the leg is at movement. When done looping, the tree is searched to get the best state, and the state with the best utility is then returned.

# Implementation
I chose to test my implementation abstractly. 

### **State**
The state is an array of random numbers.

### **Mutations**
The possible mutations are as follows for each value in the state array. **Note:** I kept in mind prime factorization when choosing these mutations.
* Add 1
* Subtract 1
* Multiply 2, 3, 5, 7 or 11
* Divide 2, 3, 5, 7 or 11

### **Utility Function**
A utility function was used to evaluate states. To make it nondifferentiable, I randomly constructed a tree (similar to a decision tree) that the state could be run through. A predicate moves the state in one direction or another at each node based on if one of the values in the state array is greater than a threshold. At each leaf is a random array of values. The utility of a state is found by moving it through the tree and then taking the dot product of it and the leaf it lands on.

### **Rollout Strategy**
The rollout strategy selects actions based on a greedy heuristic. This heuristic returns the max utility of all the states explored when doing a breadth-first search from each action to a depth. In practice however, this strategy was not performant, so mutations were selected based on the utility of the state that it directly led to. If you wish to increase the search depth passed a depth of 0, you can do so in `implementations\constants\HEURISTIC_SEARCH_DEPTH`. To encourage more exploration, I added an epsilon value that makes it choose a random action a percentage of the time during the rollout.

### **Other Special Considerations**
So that MCTS would not get stuck on a local maximum, the algorithm did not consider mutations that would lead to states that had already been explored when applied to the current state. These already visited states were kept track of in a hashset. When the algorithm explores a node where every state leading from it has already been visited, it will backpropagate a utility value that is less than the node's average. This way, MCTS will move away from that part of the tree.

# Results
Here are the results from the last 10 runs with a rollout depth of 40 and 100000 iterations. I chose to compare picking nodes in MCTS randomly vs. using UCB. As you can see, using UCB outperformed randomly choosing the states. What's especially notable is that the random search used much more memory than using UCB - as seen by the size of the trees. Note, when the action space was only add and subtract by one, it significantly outperformed the random search to a much greater extent. This makes sense as it could capitalize off the linearity of the dot product function in local areas in the state space more effectively than randomly searching. However, also keep in mind that the random search was much faster than MCTS using UCB because it did not have to do a playoff strategy. For that reason, randomly searching could be more efficient in terms of time complexity on average.

### **Output Format**

START: { start state utility )

AFTER SMART SEARCH: { state utility after smart search }
WITH A TREE SIZE OF: { number of nodes in search tree after smart search }

AFTER RANDOM SEARCH: { state utility after random search }
WITH A TREE SIZE OF: { number of nodes in search tree after random search }

_______________________________

START: -48442113

AFTER SMART SEARCH: 61752806
WITH A TREE SIZE OF: 554326

AFTER RANDOM SEARCH: 47783066
WITH A TREE SIZE OF: 674045

_______________________________

START: -293256

AFTER SMART SEARCH: 55941941
WITH A TREE SIZE OF: 504397

AFTER RANDOM SEARCH: 52397138
WITH A TREE SIZE OF: 685682

_______________________________

START: -2024682

AFTER SMART SEARCH: 69758204
WITH A TREE SIZE OF: 239674

AFTER RANDOM SEARCH: 64045661
WITH A TREE SIZE OF: 727309

_______________________________

START: 30007841

AFTER SMART SEARCH: 76650000
WITH A TREE SIZE OF: 519252

AFTER RANDOM SEARCH: 70803037
WITH A TREE SIZE OF: 740935

_______________________________

START: -10995381

AFTER SMART SEARCH: 68756507
WITH A TREE SIZE OF: 503808

AFTER RANDOM SEARCH: 47697486
WITH A TREE SIZE OF: 699505

_______________________________

START: 3373254

AFTER SMART SEARCH: 54726790
WITH A TREE SIZE OF: 390581

AFTER RANDOM SEARCH: 28064957
WITH A TREE SIZE OF: 764491

_______________________________

START: 6390746

AFTER SMART SEARCH: 53923088
WITH A TREE SIZE OF: 526864

AFTER RANDOM SEARCH: 46342947
WITH A TREE SIZE OF: 670590

_______________________________

START: 14557786

AFTER SMART SEARCH: 66511117
WITH A TREE SIZE OF: 557862

AFTER RANDOM SEARCH: 62075797
WITH A TREE SIZE OF: 796676

_______________________________

START: 8863051

AFTER SMART SEARCH: 56495731
WITH A TREE SIZE OF: 284735

AFTER RANDOM SEARCH: 46994901
WITH A TREE SIZE OF: 776321

_______________________________

START: 31875071

AFTER SMART SEARCH: 51856421
WITH A TREE SIZE OF: 238022

AFTER RANDOM SEARCH: 51102237
WITH A TREE SIZE OF: 671463
