import java.util.LinkedList;

class AsteroidCollision_735 {
    public int[] asteroidCollision(int[] asteroids) {
        LinkedList<Integer> res = new LinkedList<>();

        for (int i = 0; i < asteroids.length; i++) {
            push(res, asteroids[i]);
        }

        int[] answer = new int[res.size()];
        int i = 0;
        for (int x : res) {
            answer[i++] = x;
        }

        return answer;
    }

    private void push(LinkedList<Integer> res, int next) {

        if (res.isEmpty()) {
            res.add(next);
            return;
        }

        int last = res.getLast();

        if (last < 0 || next > 0) {
            res.add(next);
            return;
        }

        if (last > -next) {
            return;
        }

        if (last == -next) {
            res.removeLast();
            return;
        }

        res.removeLast();
        push(res, next);
    }
}