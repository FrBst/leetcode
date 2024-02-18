import java.util.Arrays;

public class SuccessfulPairsOfSpellsAndPotions_2300 {
    public int[] successfulPairs(int[] spells, int[] potions, long success) {
        Arrays.sort(potions);

        int[] pairs = new int[spells.length];
        for (int i = 0; i < spells.length; i++) {
            pairs[i] = potions.length - find(spells[i], potions, success);
        }

        return pairs;
    }

    private int find(long spell, int[] potions, long success) {
        int left = 0;
        int right = potions.length - 1;

        while (left < right) {
            int mid = (left + right) / 2;

            if (potions[mid] * spell >= success) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }

    public static void main(String[] args) {
        new SuccessfulPairsOfSpellsAndPotions_2300().successfulPairs(new int[] {5,1,3}, new int[] {1,2,3,4,5}, 7);
    }
}
