import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import common.TreeNode;

public class AllPossibleBinaryTrees_894 {
    public List<TreeNode> allPossibleFBT(int n) {

        if (n % 2 == 0) {
            return List.of();
        }

        List<List<TreeNode>> dp = new ArrayList<>();
        for (int i = 0; i < n + 1; i++) {
            dp.add(new LinkedList<>());
        }

        dp.get(1).add(new TreeNode());

        for (int len = 3; len <= n; len += 2) {
            int sum = len - 1;

            for (int first = 1; first < sum; first += 2) {
                int second = sum - first;
                dp.get(len).addAll(getAllPermutations(dp.get(first), dp.get(second))); 
            }
        }

        return dp.get(n);
    }

    private List<TreeNode> getAllPermutations(List<TreeNode> list1, List<TreeNode> list2) {
        List<TreeNode> res = new LinkedList<>();
        for (TreeNode n1 : list1) {
            for (TreeNode n2 : list2) {
                res.add(new TreeNode(0, n1, n2));
            }
        }

        return res;
    }
}