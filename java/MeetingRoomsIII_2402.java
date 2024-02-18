import java.util.Arrays;
import java.util.Comparator;

public class MeetingRoomsIII_2402 {
    public int mostBooked(int n, int[][] meetings) {
        long[] bookedUntil = new long[n];
        int[] count = new int[n];

        Arrays.sort(meetings, Comparator.comparingInt(a -> a[0]));

        for (int[] m : meetings) {
            book(bookedUntil, count, m);
        }

        int max = 0;
        for (int i = 1; i < n; i++) {
            if (count[i] > count[max]) {
                max = i;
            }
        }

        return max;
    }

    private void book(long[] bookedUntil, int[] count, int[] m) {
        int minNumber = 0;

        for (int i = 0; i < count.length; i++) {
            if (m[0] >= bookedUntil[i]) {
                bookedUntil[i] = m[1];
                count[i]++;
                return;
            }

            if (bookedUntil[i] < bookedUntil[minNumber]) {
                minNumber = i;
            }
        }

        bookedUntil[minNumber] += (m[1] - m[0]);
        count[minNumber]++;
    }
}
