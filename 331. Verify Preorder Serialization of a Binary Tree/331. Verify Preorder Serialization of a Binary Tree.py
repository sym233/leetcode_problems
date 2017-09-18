class Solution(object):
  def isValidSerialization(self, preorder):
    """
    :type preorder: str
    :rtype: bool
    """
    if preorder == '' or preorder == '#':
      return True
    
    arr = preorder.split(',')
    
    available = 1
    
    for c in arr:
      
      if c == '#':
        available -= 1
        if available < 0:
          return False
      else:
        if available <= 0:
          return False
        else:
          available += 1
    return available == 0
