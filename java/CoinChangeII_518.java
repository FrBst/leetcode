class CoinChangeII_518 {
    public static int change(int amount, int[] coins) {
        int[] dp = new int[amount+1];
        dp[0] = 1;

        for (int coin : coins) {
            for (int sum = coin; sum <= amount; sum++) {
                if (dp[sum - coin] > 0) {
                    dp[sum] += dp[sum - coin];
                }
            }
        }

        return dp[amount];
    }

    public static void main(String[] args) {
        change(5, new int[] {1,2,5});
    }
}