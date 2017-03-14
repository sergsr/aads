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
  ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
    // more robust and allows recursive cheat at end (?)
    if (!l1) return l2;
    if (!l2) return l1;

    ListNode* resultHead = new ListNode(0);
    ListNode* resultCurrent = resultHead;

    // we will need this later
    int carry = 0;
    while (l1 && l2) {
      // do summation
      int sum = l1->val + l2->val + resultCurrent->val;
      carry = sum / 10;
      resultCurrent->val = sum % 10;

      // there are more digits left to sum
      if (carry || l1->next && l2->next) {
        resultCurrent->next = new ListNode(carry);
        resultCurrent = resultCurrent->next;
      }

      l1 = l1->next;
      l2 = l2->next;
    }

    if (l1 || l2) {
      if (!carry) {
        resultCurrent->next = new ListNode(0);
        resultCurrent = resultCurrent->next;
      }

      ListNode* leftover = l1 ? l1 : l2;
      while (leftover) {
        int sum = leftover->val + resultCurrent->val;
        carry = sum / 10;
        resultCurrent->val = sum % 10;

        // there are more digits left to sum
        if (carry || leftover->next) {
          resultCurrent->next = new ListNode(carry);
          resultCurrent = resultCurrent->next;
        }

        leftover = leftover->next;
        }
      }

    return resultHead;
  }
};
