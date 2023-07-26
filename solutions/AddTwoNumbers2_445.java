import common.ListNode;

public class AddTwoNumbers2_445 {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {

        // Calculate difference between the lists' lengths
        ListNode ptr1 = l1;
        ListNode ptr2 = l2;
        int balance = 0;
        while (ptr1.next != null || ptr2.next != null) {
            if (ptr1.next != null) {
                balance++;
                ptr1 = ptr1.next;
            }
            if (ptr2.next != null) {
                balance--;
                ptr2 = ptr2.next;
            }
        }

        ptr1 = l1;
        ptr2 = l2;
        if (balance > 0) {
            ptr2 = padded(balance, l2);
        }

        if (balance < 0) {
            ptr1 = padded(-balance, l1);
        }

        ListNode sum = new ListNode();

        add(sum, ptr1, ptr2);
        return sum.val == 0 ? sum.next : sum;
    }

    private ListNode padded(int length, ListNode addAtEnd) {

        if (length == 0) {
            return addAtEnd;
        }

        return new ListNode(0, padded(length-1, addAtEnd));
    }

    // Recursive method writes sum of l1 and l2 to head. Requires l1 and l2 of equal length
    private void add(ListNode head, ListNode l1, ListNode l2) {

        ListNode sum = new ListNode(l1.val + l2.val);
        head.next = sum;

        if (l1.next != null) {
            add(sum, l1.next, l2.next);
        }

        if (sum.val > 9) {
            sum.val -= 10;
            head.val += 1;
        }
    }
}