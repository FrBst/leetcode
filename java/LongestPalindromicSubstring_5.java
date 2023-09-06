public class LongestPalindromicSubstring_5 {
    public String longestPalindrome(String s) {
        boolean[][] dp = new boolean[s.length()][s.length()];

        int maxStart = 0;
        int maxEnd = 0;
        int max = 1;

        for (int len = 0; len < s.length(); len++) {
            for (int start = 0; start < s.length(); start++) {

                if (len == 0) {
                    dp[start][start] = true;
                    continue;
                }

                int end = start + len;

                if (end >= s.length()) {
                    continue;
                }

                if ((len == 1 || dp[start+1][end-1] == true) && s.charAt(start) == s.charAt(end)) {
                    dp[start][end] = true;
                    if (len + 1 > max) {
                        max = len + 1;
                        maxStart = start;
                        maxEnd = end;
                    }
                    continue;
                }
            }
        }

        return s.substring(maxStart, maxEnd + 1);
    }
}
