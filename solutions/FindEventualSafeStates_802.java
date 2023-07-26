import java.util.LinkedList;
import java.util.List;

public class FindEventualSafeStates_802 {
    public List<Integer> eventualSafeNodes(int[][] graph) {
        // 0 for unknown, -1 for unsafe, 1 for safe
        int[] state = new int[graph.length];

        for(int i = 0; i < graph.length; i++) {
            isSafe(graph, state, i);
        }

        List<Integer> ans = new LinkedList<>();
        for (int i = 0; i < state.length; i++) {
            if (state[i] == 1) {
                ans.add(i);
            }
        }

        return ans;
    }

    private boolean isSafe(int[][] graph, int[] state, int node) {
        // Is it verified safe?
        if (state[node] == 1) {
            return true;
        }

        // Is it verified unsafe?
        if (state[node] == -1) {
            return false;
        }

        // Mark it as unsafe when result is not yet known to avoid loops
        state[node] = -1;

        // Return false if at least one destination is unsafe
        for (int i = 0; i < graph[node].length; i++) {

            int dest = graph[node][i];

            if (!isSafe(graph, state, dest)) {
                return false;
            }
        }

        // If all destinations are safe, cache and return `true`
        state[node] = 1;
        return true;
    }
}