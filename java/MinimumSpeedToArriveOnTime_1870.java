public class MinimumSpeedToArriveOnTime_1870 {
    public int minSpeedOnTime(int[] dist, double hour) {
        int minSpeed = 1;
        int maxSpeed = 10_000_000;

        while (minSpeed < maxSpeed) {
            int mid = (minSpeed + maxSpeed) / 2;

            if (calculateTime(dist, mid) > hour) {
                minSpeed = mid + 1;
                continue;
            }

            maxSpeed = mid;
        }

        return calculateTime(dist, minSpeed) <= hour ? minSpeed : -1;
    }

    private double calculateTime(int[] dist, int speed) {
        double time = 0;
        for (int x : dist) {
            time = Math.ceil(time);
            time += (double) x / speed;
        }
        return time;
    }

    public static void main(String[] args) {
        new MinimumSpeedToArriveOnTime_1870().minSpeedOnTime(new int[] {1,3,2}, 2.7);
    }
}