import java.util.HashSet;
import java.util.Set;

public class LargestPositiveIntegerThatExistsWithItsNegative_2441 {
    public int findMaxK(int[] nums) {
        Set<Integer> set = new HashSet<>();
        for (int x : nums) {
            set.add(x);
        }

        int max = Integer.MIN_VALUE;
        for (int x : nums) {
            if (x > 0 && set.contains(-x)) {
                max = Math.max(max, x);
            }
        }

        return max == Integer.MIN_VALUE ? -1 : max;
    }
}
