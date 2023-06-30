public class MedianOfTwoSortedArrays_4 {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        int totalLength = nums1.length + nums2.length;

        int ptr1 = 0;
        int ptr2 = 0;
        double median = 0.0;
        while (ptr1 < nums1.length || ptr2 < nums2.length) {

            int value1 = ptr1 < nums1.length ? nums1[ptr1] : Integer.MAX_VALUE;
            int value2 = ptr2 < nums2.length ? nums2[ptr2] : Integer.MAX_VALUE;

            // The left number if overall count is even, or the median if overall count is odd
            if (ptr1 + ptr2 == (totalLength - 1) / 2) {
                median += Math.min(value1, value2);
            // The right number if overall count is even, skipped if overall count is odd
            } else if (ptr1 + ptr2 == totalLength / 2) {
                median += Math.min(value1, value2);
                break;
            }

            if (value1 < value2) {
                ptr1++;
            } else {
                ptr2++;
            }
        }

        if (totalLength % 2 == 1) {
            return median;
        } else {
            return median / 2.0;
        }
    }
}