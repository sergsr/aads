/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
public class Solution {
  public ListNode rotateRight(ListNode head, int k) {
    if (head == null || head.next == null || k == 0) {
      return head;
    }

    ListNode left = head;
    ListNode right = head;

    for (int i = 0; i < k; ++i) {
      right = right.next;
      if (right == null) {
        right = head;
        for (int j = 0; j < (k % (i + 1)); ++j) {
          right = right.next;
        }
        break;
      }
    }

    while (right.next != null) {
      right = right.next;
      left = left.next;
    }

    right.next = head;
    head = left.next;
    left.next = null;
    return head;
  }
}
