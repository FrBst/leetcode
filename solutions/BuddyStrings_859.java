public class BuddyStrings_859 {
    public boolean buddyStrings(String s, String goal) {

        if (s.length() != goal.length() || s.length() < 2) {
            return false;
        }

        if (s.equals(goal)) {
            return f1(s);
        } else {
            return f2(s, goal);
        }
    }

    private boolean f1(String str) {
        boolean[] charEncountered = new boolean[26];

        for (char c : str.toCharArray()) {
            int index = c - 'a';

            if (!charEncountered[index]) {
                charEncountered[index] = true;
            } else {
                return true;
            }
        }

        return false;
    }

    private boolean f2(String s1, String s2) {
        int first = -1;
        int second = -1;

        for (int i = 0; i < s1.length(); i++) {
            if (s1.charAt(i) == s2.charAt(i)) {
                continue;
            }

            if (first == -1) {
                // Save first encounter
                first = i;
                continue;
            }
            
            if (second == -1) {
                // Check if second encounter counters the first
                if (s1.charAt(i) != s2.charAt(first) || s1.charAt(first) != s2.charAt(i)) {
                    return false;
                }

                second = i;
                continue;
            }
            
            // That means we need to swap more than two letters
            return false;
        }

        return second != -1;
    }
}
