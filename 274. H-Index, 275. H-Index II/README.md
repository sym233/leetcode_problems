#274. H-Index

For n citations, maximun H-index is n and minimum is 0.

Sort array citations into increasing order.

if citations[i] >= l-i, citations[i:] >= l-i and citations[:i] are not more than l-i. So l-i is a valid H-index. To find the maximum H-index, just let i from 0 to l and do the judgement. If no H-index matched, it is 0.

For 275. H-Index II, the citations is sorted, simplely delete line 6 and it's accepted.