import java.util.stream.IntStream;

class Ex1 {
	public static void main(String[] args) {
		int s = IntStream.range(1, 1000).filter(x -> x % 3 == 0 || x % 5 == 0).sum();
		System.out.println(s);
	}
}
