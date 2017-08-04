/**
 * Initialize your data structure here.
 */

function char2int(char){
  // let a -> 1, b -> 2...
  if(typeof char === 'string' && char.length === 1){
      
  }else{
      throw new Error('invalid char');
  }
  const a = 'a'.charCodeAt(0);
  return char.charCodeAt(0) - a + 1;
}

class TrieNode{
  constructor(){
    this.children = [];
  }
  
  insertStrAtThis(str){
    if(str.length === 0){
      this.children[0] = null;
    }else{
      const c = str[0];
      const i = char2int(c);
      if(this.children[i] instanceof TrieNode){
      }else{
        this.children[i] = new TrieNode();
      }
      this.children[i].insertStrAtThis(str.slice(1));
    }
  }
  
  searchStrAtThis(str, requireEnd){
    if(str.length === 0){
      if(requireEnd){
        return this.children[0] === null;
      }else{
        return true;
      }
    }else{
      const c = str[0];
      const i = char2int(c);
      if(this.children[i] instanceof TrieNode){
        return this.children[i].searchStrAtThis(str.slice(1), requireEnd);
      }else{
        return false;
      }
    }
  }
}

class Trie{
  constructor(){
    this.root = new TrieNode();
  }
/**
 * Inserts a word into the trie. 
 * @param {string} word
 * @return {void}
 */
  insert(word){
    this.root.insertStrAtThis(word);
  }
/**
 * Returns if the word is in the trie. 
 * @param {string} word
 * @return {boolean}
 */
  search(word){
    return this.root.searchStrAtThis(word, true);
  }
    
/**
 * Returns if there is any word in the trie that starts with the given prefix. 
 * @param {string} prefix
 * @return {boolean}
 */
  startsWith(prefix){
    return this.root.searchStrAtThis(prefix, false);
  }
}


/** 
 * Your Trie object will be instantiated and called as such:
 * var obj = Object.create(Trie).createNew()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */
