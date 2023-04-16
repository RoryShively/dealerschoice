# Dealers Chioce

This is a silly small utility that allows you to enter various choices via a command prompt and have one chosen at random for you.

TODO feature list:
- improve cmd line UI
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
q

Current choices: ["q"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1
Enter new choice:
w

Current choices: ["q", "w"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1e
Incorrect input submitted

Current choices: ["q", "w"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1
Enter new choice:
r

Current choices: ["q", "w", "r"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1t
Incorrect input submitted

Current choices: ["q", "w", "r"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

1
Enter new choice:
y

Current choices: ["q", "w", "r", "y"]
Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice

3
r
```