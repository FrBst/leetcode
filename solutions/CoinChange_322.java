import java.util.Arrays;

class CoinChange_322 {
    public static int coinChange(int[] coins, int amount) {
        final int VOID = 1000 + 2;
        int[] dp = new int[amount+1];
        Arrays.fill(dp, VOID);
        dp[0] = 0;

        for (int sum = 1; sum <= amount; sum++) {
            for (int coin : coins) {
                if (sum - coin >= 0) {
                    dp[sum] = Math.min(dp[sum], dp[sum - coin] + 1);
                }
            }
        }

        return dp[amount] == VOID ? -1 : dp[amount];
    }

    public static void main(String[] args) {
        coinChange(new int[] {1}, 10000);
    }
}