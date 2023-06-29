import java.util.ArrayList;
import java.util.List;
import java.util.PriorityQueue;

public class FindKPairsWithSmallestSums_373 {
    public List<List<Integer>> kSmallestPairs(int[] nums1, int[] nums2, int k) {

        PriorityQueue<List<Integer>> queue = new PriorityQueue<>(
            nums1.length,
            (pair1, pair2) -> Integer.compare(nums1[pair1.get(0)] + nums2[pair1.get(1)], nums1[pair2.get(0)] + nums2[pair2.get(1)])
            );

        for (int i = 0; i < nums1.length; i++) {
            queue.offer(List.of(i, 0));
        }
        
        List<List<Integer>> result = new ArrayList<>(k);

        while (result.size() < k) {
            List<Integer> indexes = queue.poll();

            if (indexes == null) {
                break;
            }

            result.add(List.of(nums1[indexes.get(0)], nums2[indexes.get(1)]));

            if (indexes.get(1) < nums2.length - 1) {
                queue.offer(List.of(indexes.get(0), indexes.get(1) + 1));
            }
        }
        
        return result;
    }
}