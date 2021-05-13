import java.util.ArrayList;
import java.util.HashMap;

public class Main {
  public static void main(String[] args) {
    HashMap<Integer, ArrayList<Integer>> map =
        new HashMap<Integer, ArrayList<Integer>>();
    ArrayList<Integer> list = new ArrayList<Integer>();

    list.add(4);
    list.add(10);

    map.put(1, list);
  }
}