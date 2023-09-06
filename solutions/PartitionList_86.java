class PartitionList_86 {
    public ListNode partition(ListNode head, int x) {

        ListNode leftHead = new ListNode();
        ListNode rightHead = new ListNode();
        ListNode leftPtr = leftHead;
        ListNode rightPtr = rightHead;

        ListNode ptr = head;
        while (ptr != null) {
            if (ptr.val < x) {
                leftPtr.next = ptr;
                leftPtr = ptr;
            } else {
                rightPtr.next = ptr;
                rightPtr = ptr;
            }
            ptr = ptr.next;
        }

        rightPtr.next = null;
        leftPtr.next = rightHead.next;
        return leftHead.next;

    }
}