### Lab 9

**Assignment Number**: 19

There are only 5 houses on the street. 
They are painted in different colors. 
They are occupied by the families of a poet, writer, critic, journalist and editor. 
Every house has a favorite bird. The head of the family gets his favorite drink for breakfast, 
after which he goes to the city, using his favorite variation of transportation. We know that:
1. The poet rides a bicycle, the editor lives in a red house.
2. The critic lives in the farthest house on the left, next to which is the blue house.
3. Someone who rides a motorcycle lives in an middle house.
4. The owner of the green house, located to the right of the white one, always goes to the city by walking.
5. In the house where the bullfinch lives, they always drink milk for breakfast.
6. The one who gets cocoa for breakfast lives next door to the house where the chickadee lives.
7. Tea is served for breakfast in the yellow house.
8. The owner who lives next to the canary lover drinks tea in the morning.
9. The writer drinks only coffee.
10. The one who drives a car likes apple juice.
11. A parrot lives in the journalist's house.

**QUESTION:** Who has a magpie?

![](./png/RESULTS.png)

**Query:**
```prolog
2 ?- task(MagpieOwner).
1) yellow, critic, chickadee, tea, UNKNOWN_TRANSPORT
2) blue, poet, canary, cocoa, bicycle
3) red, editor, bullfinch, milk, motorcycle
4) white, journalist, parrot, apple_juice, car
5) green, writer, magpie, coffee, walking

MagpieOwner = house(green, writer, magpie, coffee, walking) ;
false.
```