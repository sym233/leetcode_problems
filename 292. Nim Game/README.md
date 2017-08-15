292. Nim Game

It's a problem of game theory. 感觉我英语真是捉鸡.

There are two players and a heap of stones. Players pick 1, 2 or 3 stones in turn. One may lose if no stone to pick.

Let's assume that the first player to pick stones named Alice and the second one Bob.

If there are 1, 2 or 3 stones, Alice can pick them all and win the game. But when there are 4 stones, no matter how many Alice pick, he will leave 1, 2 or 3 stones for Bob and then lose the game. If there are 5 stones, Alice can pick 1 stone and make Bob lost.

We can find that:

stones:   Winner:

0         Bob

1         Alice

2         Alice

3         Alice

4         Bob

5         Alice

...

When n % 4 !== 0, Alice can pick some stones makes the number of remaining stone is a multiple of 4, and make Bob lost.

Then realize that Alice will win when n % 4 !== 0, otherwise Bob will win.

But what the faq my program is much slower than other's even if there is only 1 statement.
