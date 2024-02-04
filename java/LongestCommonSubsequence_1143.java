public class LongestCommonSubsequence_1143 {
    public int longestCommonSubsequence(String text1, String text2) {
        return dp(text1, text2);
    }

    public int dp(String text1, String text2) {

        int[] dp = new int[text1.length() + 1];

        for (int i = 1; i <= text2.length(); i++) {
            int prev = 0;
            for (int j = 1; j <= text1.length(); j++) {
                int temp = dp[j];
                if (text1.charAt(j - 1) == text2.charAt(i - 1)) {
                    dp[j] = prev + 1;
                } else {
                    dp[j] = Math.max(dp[j - 1], dp[j]);
                }

                prev = temp;
            }
        }

        return dp[text1.length()];
    }
}
