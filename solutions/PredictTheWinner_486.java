public class PredictTheWinner_486 {
    public boolean PredictTheWinner(int[] nums) {
        int[][] score = new int[nums.length][nums.length];


        for (int len = 0; len < nums.length; len++) {
            for (int start = 0; start < nums.length - len; start++) {
                if (len == 0) {
                    score[start][start] = nums[start];
                    continue;
                }

                int end = start + len;

                int var1 = nums[start] - score[start+1][end];
                int var2 = nums[end] - score[start][end-1];

                score[start][end] = Math.max(var1, var2);
            }
        }

        return score[0][nums.length-1] >= 0;
    }
}
