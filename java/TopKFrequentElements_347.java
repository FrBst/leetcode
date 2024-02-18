import java.util.Arrays;

public class TopKFrequentElements_347 {
        public int[] topKFrequent(int[] nums, int k) {
        int i = quickselect(nums, 0, nums.length - 1, k);

        return Arrays.copyOfRange(nums, k, nums.length);
    }

    private int quickselect(int[] arr, int lo, int hi, int k) {
        int mid = partition(arr, lo, hi);

        if (hi - mid + 1 == k) {
            return mid;
        }
        if (hi - mid + 1 > k) {
            return quickselect(arr, mid, hi, k);
        }

        return quickselect(arr, lo, mid - 1, k - (hi - mid + 1));
    }

    private int partition(int[] arr, int lo, int hi) {
        int pivot = arr[(lo + hi) / 2];
        lo--;
        hi++;

        while (true) {
            do {
                lo++;
            } while (arr[lo] < pivot);
            do {
                hi--;
            } while (arr[hi] > pivot);

            if (lo >= hi) {
                return hi;
            }

            int temp = arr[lo];
            arr[lo] = arr[hi];
            arr[hi] = temp;
        }
    }

    public static void main(String[] args) {
        new TopKFrequentElements_347().topKFrequent(new int[] {2,4,9,3,7,8,4,0,1,7,5,4,9,6,3}, 5);
    }
}
