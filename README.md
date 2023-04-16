# Dealers Chioce

This is a silly small utility that allows you to enter various choices via a command prompt and have one chosen at random for you.

TODO feature list:
- dedup choices
- add testing
- better error handling

Example use:
```
Current choices: []
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1
Enter new choice:
get x

Current choices: ["get x"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1
Enter new choice:
get y

Current choices: ["get x", "get y"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice
```