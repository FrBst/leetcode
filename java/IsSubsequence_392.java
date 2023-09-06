public class IsSubsequence_392 {
    public boolean isSubsequence(String s, String t) {

        int sIndex = 0;

        for (int tIndex = 0; tIndex < t.length() && sIndex < s.length(); tIndex++) {
            if (s.charAt(sIndex) == t.charAt(tIndex)) {
                sIndex++;
            }
        }

        if (sIndex != s.length()) {
            return false;
        } else {
            return true;
        }
    }
}
