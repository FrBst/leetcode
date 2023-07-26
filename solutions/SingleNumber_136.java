public class SingleNumber_136 {
    public int singleNumber(int[] nums) {
        int number = 0;

        for (int x : nums) {
            // Repeating numbers will cancel out each other
            number = number ^ x;
        }

        return number;
    }
}