/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
 public:
  ListNode *mergeTwoLists(ListNode *l1, ListNode *l2) {
    if (!l1) return l2;
    if (!l2) return l1;

    bool inOrder = l1->val < l2->val;
    ListNode *head = inOrder ? l1 : l2;
    if (inOrder) {
      l1 = l1->next;
    } else {
      l2 = l2->next;
    }

    ListNode *current = head;
    while (l1 && l2) {
      if (l1->val < l2->val) {
        current->next = l1;
        l1 = l1->next;
      } else {
        current->next = l2;
        l2 = l2->next;
      }
      current = current->next;
    }

    if (l1) {
      current->next = l1;
    } else if (l2) {
      current->next = l2;
    }

    return head;
  }
};
