import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class MinimumWindowSubstring_76 {
    public String minWindow(String s, String t) {
        Map<Character,Integer> chars = new HashMap<>();
        Set<Character> target = new HashSet<>();
        int charsLeft = 0;
        String res = "";

        for (char c : t.toCharArray()) {
            chars.merge(c, 1, Integer::sum);
            target.add(c);
            charsLeft++;
        }

        int start = 0;
        int end = 0;
        while (end < s.length()) {
            char c = s.charAt(end);
            chars.merge(c, -1, Integer::sum);
            if (chars.get(c) >= 0 && target.contains(c)) {
                charsLeft--;
            }

            if (charsLeft != 0) {
                end++;
                continue;
            }

            while (start <= end && charsLeft <= 0) {

                if (res.equals("") || res.length() > end - start) {
                    res = s.substring(start, end + 1);
                }

                c = s.charAt(start);
                chars.merge(c, 1, Integer::sum);
                if (chars.get(c) > 0 && target.contains(c)) {
                    charsLeft++;
                }
                start++;
            }

            end++;
        }

        return res;
    }

    public static void main(String[] args) {
        MinimumWindowSubstring_76 obj = new MinimumWindowSubstring_76();
        System.out.println(obj.minWindow("ADOBECODEBANC", "ABC"));
        System.out.println(obj.minWindow("a", "aa"));
    }
}
