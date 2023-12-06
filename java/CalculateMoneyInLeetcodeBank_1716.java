class CalculateMoneyInLeetcodeBank_1716 {
    public int totalMoney(int n) {
        int res = 0;
        int weeks = n / 7;
        int days = n % 7;

        res += (2 * 28 + 7 * weeks - 7) * weeks / 2;
        res += (2 * (weeks + 1) + (days - 1)) * days / 2;

        return res;
    }
}