public class MirrorReflection_858 {
    public int mirrorReflection(int p, int q) {
        int gcd = gcd(p, q);
        int lcm = p * q / gcd;

        if ((lcm / q) % 2 == 0) {
            return 2;
        }
        if ((lcm / p) % 2 == 0) {
            return 0;
        }

        return 1;
    }

    private int gcd(int a, int b) {
        while (a != b) {
            a = a > b ? a - b : a;
            b = b > a ? b - a : b;
        }

        return a;
    }
}
