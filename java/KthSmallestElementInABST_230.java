import java.util.ArrayDeque;
import java.util.Deque;

import common.TreeNode;

public class KthSmallestElementInABST_230 {
    public int kthSmallest(TreeNode root, int k) {
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.add(root);

        while (!stack.isEmpty()) {
            TreeNode node = stack.peek();
            if (node.left != null) {
                stack.push(node.left);
                continue;
            }
            if (k == 1) {
                return node.val;
            }
            stack.pop();
            k--;
            if (node.right != null) {
                stack.push(node.right);
            }
        }

        return -1;
    }

    public static void main(String[] args) {
        TreeNode[] nodes = new TreeNode[7];
        for (int i = 0; i < 7; i++) {
            nodes[i] = new TreeNode(i);
        }

        nodes[5].left = nodes[3];
        nodes[5].right = nodes[6];
        nodes[3].left = nodes[2];
        nodes[3].right = nodes[4];
        nodes[2].left = nodes[1];

        new KthSmallestElementInABST_230().kthSmallest(nodes[5], 3);
    }
}
