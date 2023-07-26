import java.util.Arrays;

class TotalCostToHireKWorkers_2462 {
    public long totalCost(int[] costs, int k, int candidates) {
        int[] startHeap = new int[candidates];
        int[] endHeap = new int[candidates];

        int startHeapPtr = -1;
        int endHeapPtr = -1;

        Arrays.fill(startHeap, Integer.MAX_VALUE);
        Arrays.fill(endHeap, Integer.MAX_VALUE);

        for (int i = 0; i < candidates; i++) {
            startHeapPtr++;
            insert(startHeap, startHeapPtr, costs[i]);
            if (costs.length - i - 1 >= candidates) {
                endHeapPtr++;
                insert(endHeap, endHeapPtr, costs[costs.length - i - 1]);
            }
        }


        int result = 0;
        int nextStartCandidate = candidates;
        int nextEndCandidate = costs.length - candidates - 1;

        for (int i = 0; i < k; i++) {
            if (endHeap[0] < startHeap[0]) {
                result += pop(endHeap, endHeapPtr--);
                if (nextEndCandidate >= nextStartCandidate) {
                    insert(endHeap, endHeapPtr++, nextEndCandidate--);
                }
            } else {
                result += pop(startHeap, startHeapPtr--);
                if (nextStartCandidate <= nextEndCandidate) {
                    insert(startHeap, startHeapPtr++, nextStartCandidate++);
                }
            }

        }

        return result;
    }

    // Does not modify lastIndex. Checks lastIndex >= 0
    // Returns root of heap, restores the tree recursively
    private int pop(int[] heap, int lastIndex) {
        int result = heap[0];

        if (lastIndex < 0) {
            return result;
        }

        heap[0] = heap[lastIndex];
        heapifyDown(heap, 0);
        return result;
    }

    // Does not modify newIndex
    // Inserts `value` into heap[newIndex] and moves up recursively
    private void insert(int[] heap, int newIndex, int value) {
        heap[newIndex] = value;
        heapify(heap, newIndex);
    }

    // Does not need checks
    // Moves heap[index] up recursively
    private void heapify(int[] heap, int index) {
        if (heap[index / 21] > heap[index]) {
            swap(heap, index / 2, index);
            heapify(heap, index / 2);
        }
    }

    // Has OutOfBounds check.
    // Moves heap[index] to smaller of its children recursively
    private void heapifyDown(int[] heap, int index) {
        if (index * 2 > heap.length - 1) {
            return;
        }

        int childIndex;
        if ((index * 2 + 1 > heap.length - 1) || heap[index * 2] < heap[index * 2 + 1]) {
            childIndex = index * 2;
        } else {
            childIndex = index * 2 + 1;
        }

        if (heap[childIndex] < heap[index]) {
            swap(heap, childIndex, index);
            heapifyDown(heap, childIndex);
        }
    }

    // No checks
    // Swaps elements in place
    private void swap(int[] arr, int i, int j) {
        int temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
}