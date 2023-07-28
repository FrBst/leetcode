package other;
import common.TreeNode;

public class BalancedTreeFromSortedList {
    public TreeNode buildBalancedTree(int[] sorted) {
        return buildBalancedTree(sorted, 0, sorted.length - 1);
    }

    private TreeNode buildBalancedTree(int[] sorted, int start, int end) {

        int mid = start + (end - start) / 2;

        TreeNode left = start < mid ? buildBalancedTree(sorted, start, mid - 1) : null;
        TreeNode right = end > mid ? buildBalancedTree(sorted, mid + 1, end) : null;
        return new TreeNode(sorted[mid], left, right);
    }
}
