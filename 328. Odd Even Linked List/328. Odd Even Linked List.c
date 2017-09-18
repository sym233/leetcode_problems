/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* oddEvenList(struct ListNode* head) {
  if(head == NULL || head->next == NULL){
    return head;
  }
  
  struct ListNode *p = head;
  
  struct ListNode *fakeHeadEven = (struct ListNode*)malloc(sizeof(struct ListNode));
  fakeHeadEven->next = NULL;
  struct ListNode *t = NULL;
  
  int i = 2;
  
  while(p->next != NULL){
    if(i & 1){
      /* is odd */
      p = p->next;
    }else{
      /* is even */
      if(t == NULL){
        fakeHeadEven->next = p->next;
        t = fakeHeadEven->next;
      }else{
        t->next = p->next;
        t = t->next;
      }
      p->next = p->next->next;
      t->next = NULL;
    }
    i++;
  }
  p->next = fakeHeadEven->next;
  
  free(fakeHeadEven);
  fakeHeadEven = t = NULL;
  
  return head;    
}
