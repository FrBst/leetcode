import java.util.Random;

class KthLargestElementInAnArray_215 {
    public static int findKthLargest(int[] nums, int k) {
        return findKthLargest(nums, k, 0, nums.length - 1);
    }

    private static int findKthLargest(int[] nums, int k, int left, int right) {

        Random rand = new Random();

        int slow = left;
        int fast = left + 1;
        int pivot = nums[rand.nextInt(left, right+1)];

        while (slow <= right && fast <= right) {
            if (nums[fast] < pivot) {
                fast++;
                continue;
            }
            if (nums[slow] >= pivot) {
                slow++;
                continue;
            }

            int temp = nums[slow];
            nums[slow] = nums[fast];
            nums[fast] = temp;
            fast++;
            slow++;
        }

        if (nums[slow] >= pivot) {
            slow++;
        }

        

        if (k < slow) {
            return findKthLargest(nums, k, left, slow-1);
        } else {
            return findKthLargest(nums, k - slow, slow, right);
        }
    }

    public static void main(String[] args) {
        int nums[] = {3,2,1,5,6,4};
        findKthLargest(nums, 2);
    }
}