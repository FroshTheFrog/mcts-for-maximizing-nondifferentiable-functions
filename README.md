# Overview
THis project served two functions. The first was that I wanted to learn the programing language Rust. The second was  to experiment with using MCTS to procedurally generate content. This would work by replacing instances where one would traditionally use evolutionary algorithms. You can see how it would work given the following example. Let’s say you wanted to construct an optimal leg for movement. In MCTS each move would be a mutation to the leg. Given that the algorithm is not running to the end of a game, the play out strategy would only run to a set depth. Rather than returning if the agent won or not, the output of a utility function at that depth is returned. In the case of a leg, this would be how effective it is at movement.

# Implementation
I chose to test my implementation abstractly. 

### **State**
The state is an array of random numbers.

### **Mutations**
The possible mutations are as follows for each value in the state. **Note:** I kept in mind prime factorization when choosing these mutations.
* Add 1
* Subtract 1
* Multiply 2 or 3 or 5 or 7 or 11
* Divide 2 or 3 or 5 or 7 or 11

### **Utility Function**
For the utility function, to make it nondifferentiable, I randomly constructed a tree that the state could be run through. A predicate moves the state in one direction or another at each node based on if one of its values is greater than a threshold. At each leaf is a random array of values. The utility of a state is found by moving it through the tree and then taking the dot product of it and its leaf.

### **Rollout Strategy**
The rollout strategy selects actions based on a greedy heuristic. This heuristic returns the max utility of all the states explored when doing a breadth-first search from each action to a depth. In practice however, this strategy was not performant, so the heuristic of each mutation was defined as the utility of the state that it led to. If you wish to increase the search depth passed a depth of 0, you can do so in `implementations\constants\HEURISTIC_SEARCH_DEPTH `. To encourage more exploration, I added an epsilon value that makes it choose a random action a percentage of the time.

### **Other Special Considerations**
* So that mcts would not get stuck on a local maximum, I did not consider mutations that would lead to states that had already been explored. This was kept track of in a hash set.
* When done looping, the tree is searched to get the best state, and the state with the best utility is returned.

# Results
Here are the results from the last ten runs with a rollout depth of 40 and 100000 iterations. As you can see, the tree search did not perform much better than randomly searching the states. Note when the action space was only add and subtract by one, it significantly outperformed the random search. This makes sense as it could capitalize off the linearity of the dot product function in local areas in the state space more effectively than randomly searching.

_______________________________

START: -36591884

AFTER SMART SEARCH: 53196443
WITH A TREE SIZE OF: 744831

AFTER RANDOM SEARCH: 53228651
WITH A TREE SIZE OF: 744283

_______________________________

START: 2740294

AFTER SMART SEARCH: 39041681
WITH A TREE SIZE OF: 716637

AFTER RANDOM SEARCH: 36075795
WITH A TREE SIZE OF: 716736

_______________________________

START: -12754592

AFTER SMART SEARCH: 58706781
WITH A TREE SIZE OF: 686352

AFTER RANDOM SEARCH: 54195433
WITH A TREE SIZE OF: 680348

_______________________________

START: 580105

AFTER SMART SEARCH: 52202410
WITH A TREE SIZE OF: 672851

AFTER RANDOM SEARCH: 51089118
WITH A TREE SIZE OF: 670935

_______________________________

START: -14259369

AFTER SMART SEARCH: 31269977
WITH A TREE SIZE OF: 663949

AFTER RANDOM SEARCH: 35180660
WITH A TREE SIZE OF: 675739

_______________________________

START: -4499691

AFTER SMART SEARCH: 36302560
WITH A TREE SIZE OF: 718495

AFTER RANDOM SEARCH: 35924416
WITH A TREE SIZE OF: 726483

_______________________________

START: 22564720

AFTER SMART SEARCH: 41122564
WITH A TREE SIZE OF: 671662

AFTER RANDOM SEARCH: 42025146
WITH A TREE SIZE OF: 674727

_______________________________

START: -12289775

AFTER SMART SEARCH: 38173292
WITH A TREE SIZE OF: 765237

AFTER RANDOM SEARCH: 37651706
WITH A TREE SIZE OF: 759507

_______________________________

START: -15923824

AFTER SMART SEARCH: 73376684
WITH A TREE SIZE OF: 688065

AFTER RANDOM SEARCH: 73376684
WITH A TREE SIZE OF: 692931

_______________________________

START: -2380563

AFTER SMART SEARCH: 45010689
WITH A TREE SIZE OF: 651814

AFTER RANDOM SEARCH: 45010689
WITH A TREE SIZE OF: 666615

_______________________________
