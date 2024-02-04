public class CountPairsOfSimilarStrings_2506 {
    public int similarPairs(String[] words) {
        int[] count = new int[words.length];

        for (int i = 0; i < words.length; i++) {
            for (char c : words[i].toCharArray()) {
                count[i] |= 1 << (c - 'a');
            }
        }

        int pairs = 0;
        for (int i = 0; i < count.length; i++) {
            for (int j = 0; j < i; j++) {
                if (count[i] == count[j]) {
                    pairs++;
                }
            }
        }

        return pairs;
    }
}
