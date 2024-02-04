class SetMismatch_645 {
    public int[] findErrorNums(int[] nums) {

        int n = nums.length;

        int diff = 0;
        // `diff` will be equivalent to ans1 ^ ans2
        for (int i = 1; i <= n; i++) {
            diff ^= i;
            diff ^= nums[i-1];
        }

        int rightmostSetBit = diff & -diff;
        int bitSet = 0;
        int bitNotSet = 0;

        // One group will have the repeating number, the other the missing number.
        for (int i = 1; i <= n; i++) {
            if ((rightmostSetBit & i) == 0) {
                bitNotSet ^= i;
            } else {
                bitSet ^= i;
            }
        }

        // Eliminate rest of numbers
        for (int num : nums) {
            if ((rightmostSetBit & num) == 0) {
                bitNotSet ^= num;
            } else {
                bitSet ^= num;
            }
        }

        for (int num : nums) {
            if (num == bitSet) {
                return new int[] { bitSet, bitNotSet };
            }
        }
        return new int[] { bitNotSet, bitSet };
    }

    public static void main(String[] args) {
        new SetMismatch_645().findErrorNums(new int[] {1, 2, 2, 4});
    }
}
