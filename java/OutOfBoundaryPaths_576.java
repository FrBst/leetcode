class OutOfBoundaryPaths_576 {

    private static int MOD = 1_000_000_007;

    public int findPaths(int m, int n, int maxMove, int startRow, int startColumn) {
        int[][][] dp = new int[m][n][maxMove + 1];
        return findPaths(dp, m, n, maxMove, startRow, startColumn);
    }

    private int findPaths(int[][][] dp, int m, int n, int maxMove, int x, int y) {

        if (x == -1 || x == m || y == -1 || y == n) {
            return 1;
        }

        int num = dp[x][y][maxMove];

        if (num == -1 || maxMove == 0) {
            return 0;
        }

        if (num > 0) {
            return num;
        }

        int res = 0;
        res = (res + findPaths(dp, m, n, maxMove - 1, x - 1, y)) % MOD;
        res = (res + findPaths(dp, m, n, maxMove - 1, x + 1, y)) % MOD;
        res = (res + findPaths(dp, m, n, maxMove - 1, x, y - 1)) % MOD;
        res = (res + findPaths(dp, m, n, maxMove - 1, x, y + 1)) % MOD;

        dp[x][y][maxMove] = res == 0 ? -1 : res;
        return res;
    }

    public static void main(String[] args) {
        OutOfBoundaryPaths_576 obj = new OutOfBoundaryPaths_576();
        System.out.println(obj.findPaths(2, 2, 2, 0, 0));
    }
}