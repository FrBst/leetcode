import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Permutations_46 {
    public static List<List<Integer>> permute(int[] nums) {

        List<List<Integer>> res = new ArrayList<>();

        if (nums.length == 1) {
            List<Integer> single = new ArrayList<>();
            single.add(nums[0]);
            res.add(single);
            return res;
        }

        for (int i = 0; i < nums.length; i++) {

            int[] minusOne = new int[nums.length-1];
            int index = 0;
            for (int j = 0; j < nums.length; j++) {
                if (j != i) {
                    minusOne[index] = nums[j];
                    index++;
                }
            }
            List<List<Integer>> permutations = permute(minusOne);

            for (List<Integer> p : permutations) {
                p.add(nums[i]);
            }

            res.addAll(permutations);
        }

        return res;
    }

    public static void main(String[] args) {
        int[] data = {1,2,3};
        permute(data);
    }
}