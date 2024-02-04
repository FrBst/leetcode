import common.TreeNode;

public class PseudoPalindromicPathsInABinaryTree_1457 {
    public int pseudoPalindromicPaths (TreeNode root) {
        return recurse(root, 0);
    }

    private int recurse(TreeNode root, int count) {

        if (root == null) {
            return 0;
        }

        count ^= 1 << root.val;

        if (root.right == null && root.left == null) {
            int oddCounts = Integer.bitCount(count);
            if (oddCounts <= 1) {
                return 1;
            }
        }

        return recurse(root.left, count) + recurse(root.right, count);
    }
}