# Dealers Chioce

This is a silly small utility that allows you to enter various choices via a command prompt and have one chosen at random for you.

TODO feature list:
- create actions such as add or remove item from choices
- dedup choices
- cleanup output
- add testing
- split into modules

Example use (Will clean this up when actions feature is completed):
```
Enter new choice (type "done" to exit):
d
input: d
len(input): 2
choices: ["d"]
len(choices): 1
exit_loop: false
Enter new choice (type "done" to exit):
e
input: e
len(input): 2
choices: ["d", "e"]
len(choices): 2
exit_loop: false
Enter new choice (type "done" to exit):
c
input: c
len(input): 2
choices: ["d", "e", "c"]
len(choices): 3
exit_loop: false
Enter new choice (type "done" to exit):
done
input: done
len(input): 5
Some("c")
```