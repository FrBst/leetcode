public class MoveZeroes_283 {
    public void moveZeroes(int[] nums) {
        int vacuum = 0;

        for (int runner = 0; runner < nums.length; runner++) {
            if (nums[runner] != 0) {
                nums[vacuum] = nums[runner];
                vacuum++;
            }
        }

        while (vacuum < nums.length) {
            nums[vacuum] = 0;
            vacuum++;
        }
    }
}
