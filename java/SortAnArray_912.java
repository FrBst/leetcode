public class SortAnArray_912 {
    public static int[] sortArray(int[] nums) {
        quicksort(nums, 0, nums.length-1);
        return nums;
    }

    private static void quicksort(int[] arr, int lo, int hi) {
        if (lo >= 0 && hi >= 0 && lo < hi) {
            int p = partition(arr, lo, hi);
            quicksort(arr, lo, p);
            quicksort(arr, p+1, hi);
        }
    }

    private static int partition(int[] arr, int lo, int hi) {
        int pivot = arr[lo + (hi - lo) / 2];
        int i = lo - 1;
        int j = hi + 1;

        while(true) {
            do {
                i++;
            } while (arr[i] < pivot);
            do {
                j--;
            } while (arr[j] > pivot);

            if (i >= j) {
                return j;
            }

            swap(arr, i, j);
        }
    }

    private static void swap(int[] arr, int a, int b) {
        int temp = arr[a];
        arr[a] = arr[b];
        arr[b] = temp;
    }

    public static void main(String[] args) {
        int[] nums = {5,2,3,1};
        sortArray(nums);
    }
}
