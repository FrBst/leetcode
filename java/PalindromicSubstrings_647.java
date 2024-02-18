import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class PalindromicSubstrings_647 {
    public int countSubstrings(String s) {
        int n = s.length();
        boolean[][] dp = new boolean[n+1][n+1];

        int res = 0;

        char[] chars = s.toCharArray();

        for (int i = 0; i < n; i++) {
            dp[i][1] = true;
            res++;
            if (i < n - 1 && chars[i] == chars[i+1]) {
                dp[i][2] = true;
                res++;
            }
        }

        for (int len = 3; len <= n; len++) {
            for (int start = 0; start <= n - len; start++) {
                if (!dp[start+1][len-2]) {
                    continue;
                }
                if (chars[start] == chars[start+len-1]) {
                    dp[start][len] = true;
                    res++;
                }
            }
        }

        return res;
    }

    public static void main(String[] args) {
        new PalindromicSubstrings_647().countSubstrings("aaa");
    }
    
}
