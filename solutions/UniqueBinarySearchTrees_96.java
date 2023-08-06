public class UniqueBinarySearchTrees_96 {
    public static int numTrees(int n) {
        int[] dp = new int[20];
        dp[0] = 1;
        
        for (int size = 1; size <= n; size++) {
            for (int pivot = 0; pivot < size; pivot++) {
                dp[size] += dp[size - pivot - 1] * dp[pivot];
            }
        }

        return dp[n];
    }

    public static void main(String[] args) {
        for (int n = 1; n <= 19; n++) {
            System.out.printf("if (n == %s) {\n", n);
            System.out.printf("    return %s;\n", numTrees(n));
            System.out.println("}");
        }
    }
}
