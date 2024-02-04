public class ReplaceAllDigitsWithCharacters_1844 {
    public String replaceDigits(String s) {
        char[] ans = s.toCharArray();

        for (int i = 1; i < s.length(); i += 2) {
            ans[i] += ans[i-1] - '0';
        }

        return new String(ans);
    }
}
