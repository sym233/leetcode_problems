https://leetcode.com/problems/text-justification

first, consider array words is empty. Just return a string with spaces.

line 12;

function count_width is to calc width of words. No word, width is 0;
 1 word, width is its length; more words, length is the sum of their lengths
 and spaces between 2 words.

p is a pointer to array words. With a word added, p increase 1.

res is the result.

Create a empty array temp_line, add words form array words as many as possible,
 until count_width not less then maxWidth. Pay attention to last word. Repeat.

 
Function make_line is to place spaces between words. n+1 words in a line, words' length is l.
  m words are after x+1 space(s) and n-m word(s) are after x space(s).
  First word don't need any space.
  so that m(x+1) + (n-m)x + l === maxWidth.
  Equals to m + nx === maxWidth - l.
  Easy to solve for m and x.
  Then form the line.
  
line 63;

Last line only need 1 space between 2 words. Fill the end with space.
