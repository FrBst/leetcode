import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import common.TreeNode;

public class UniqueBinarySearchTrees_II_96 {
    
    public List<TreeNode> generateTrees(int n) {
        List<List<TreeNode>> dp = new ArrayList<>();
        dp.add(null);
        dp.add(List.of(new TreeNode(1)));

        for (int size = 2; size <= n; size++) {
            dp.add(new ArrayList<>());
            for (int pivot = 1; pivot <= size; pivot++) {
                List<TreeNode> right = incAll(dp.get(size - pivot - 1), pivot);
                List<TreeNode> left = incAll(dp.get(pivot - 1), 0);

                for (TreeNode l : left) {
                    for (TreeNode r : right) {
                        dp.get(size).add(new TreeNode(pivot, l, r));
                    }
                }
            }
        }

        return dp.get(n);
    }

    private List<TreeNode> incAll(List<TreeNode> prototypes, int n) {
        if (prototypes == null) {
            return Collections.singletonList(null);
        }
        List<TreeNode> res = new ArrayList<>(prototypes.size());
        for (TreeNode tree : prototypes) {
            res.add(inc(tree, n));
        }
        return res;
    }

    private TreeNode inc(TreeNode prototype, int n) {
        if (prototype == null) {
            return null;
        }
        TreeNode res = new TreeNode(prototype.val + n);
        res.left = inc(prototype.left, n);
        res.right = inc(prototype.right, n);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(new UniqueBinarySearchTrees_II_96().generateTrees(2));
    }
}