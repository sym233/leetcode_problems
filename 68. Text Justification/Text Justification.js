/**
 * @param {string[]} words
 * @param {number} maxWidth
 * @return {string[]}
 */
var fullJustify = function(words, maxWidth) {
    if(words.find(word=>word.length > maxWidth)){
        throw new Error('word to long');
    }
    if(words.length === 0){
        return [' '.repeat(maxWidth)];
    }
    
    let p = 0;
    let res = [];
    
    function count_width(arr){
        // count words' width, a word is with a space
        // console.log(arr);
        if(arr.lenth === 0){
            return 0;
        }
        return arr.reduce((accu, item)=>accu+item.length+1, -1);
    }
    
    function make_line(arr){
        if(arr.length === 1){
            return arr[0] + ' '.repeat(maxWidth-arr[0].length);
        }
        
        let l = arr.reduce((accu, item)=>accu+item.length, 0);
        // split a line
        // n+1 words in a line, lenght = l
        // m words after x+1 space(s) and n-m word(s) after x space(s)
        // so that m(x+1) + (n-m)x + l === maxWidth
        
        let n = arr.length - 1;
        let m = (maxWidth-l) % n;
        let x = Math.floor((maxWidth-l) / n);
        return arr.reduce((accu, item, i)=>{
            if(i <= m){
                return accu + ' '.repeat(x+1) + item;
            }else{
                return accu + ' '.repeat(x) + item;
            }
        });
    }
    
    while(p < words.length){
        let temp_line = [];
        
        while(count_width(temp_line.concat(words[p])) <= maxWidth){
            temp_line.push(words[p]);
            p++;
            if(p === words.length){
                break;
            }
        }
        
        if(temp_line.length !== 0){
            res.push(make_line(temp_line));
        }
    }
    
    // process last line
    let last_line = res.pop();
    last_line = last_line.replace(/\s{2,}/g, ' ');
    last_line = last_line + ' '.repeat(maxWidth-last_line.length);
    res.push(last_line);
    
    return res;
};