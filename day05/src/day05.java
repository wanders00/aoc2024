import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.io.IOException;
import java.util.HashMap;
import java.util.HashSet;

public class day05 {
    public static ArrayList<Integer> invalid_ordered = new ArrayList<>();
    public static HashMap<Integer, ArrayList<Integer>> ruleMap = new HashMap<>();

    public static void main(String[] args) {
        long startTime, endTime, duration;
        double durationInSeconds;
        startTime = System.nanoTime();

        ArrayList<ArrayList<Integer>> rules = new ArrayList<ArrayList<Integer>>();
        ArrayList<ArrayList<Integer>> order = new ArrayList<ArrayList<Integer>>();
        try {
            String[] input = {}, part1, part2;
            input = new String(Files.readAllBytes(Paths.get("day05/src/input.txt"))).split("\r\n\r\n");
            part1 = input[0].split("\n");
            part2 = input[1].split("\n");

            part1 = input[0].split("\n");
            part2 = input[1].split("\n");

            for (int i = 0; i < part1.length; i++) {
                String[] temp = part1[i].split("\\|");
                ArrayList<Integer> rule = new ArrayList<Integer>();
                for (int j = 0; j < temp.length; j++) {
                    rule.add(Integer.parseInt(temp[j].trim()));
                }
                rules.add(rule);
            }

            for (int i = 0; i < part2.length; i++) {
                String[] temp = part2[i].split(",");
                ArrayList<Integer> line = new ArrayList<Integer>();
                for (int j = 0; j < temp.length; j++) {
                    line.add(Integer.parseInt(temp[j].trim()));
                }
                order.add(line);
            }

        } catch (IOException e) {
            e.printStackTrace();
        }

        for (ArrayList<Integer> rule : rules) {
            if (ruleMap.containsKey(rule.get(0))) {
                ruleMap.get(rule.get(0)).add(rule.get(1));
            } else {
                ArrayList<Integer> temp = new ArrayList<>();
                temp.add(rule.get(1));
                ruleMap.put(rule.get(0), temp);
            }
        }

        endTime = System.nanoTime();
        duration = (endTime - startTime); // in nanoseconds
        durationInSeconds = duration / 1_000_000_000.0;
        System.out.println("Time taken - Loading & Processing input: " + durationInSeconds + " seconds");

        // Part 1
        startTime = System.nanoTime();

        int correct_orders = 0;
        for (int k = 0; k < order.size(); k++) {
            ArrayList<Integer> line = order.get(k);
            boolean invalid = false;
            HashSet<Integer> seen = new HashSet<>();
            for (int i = 0; i < line.size(); i++) {
                if (invalid) {
                    break;
                }

                int current = line.get(i);

                if (!ruleMap.containsKey(current)) {
                    seen.add(current);
                    continue;
                }

                for (Integer rule : ruleMap.get(current)) {
                    if (seen.contains(rule)) {
                        invalid = true;
                        break;
                    }
                }

                seen.add(current);
            }

            if (invalid) {
                invalid_ordered.add(k);
                continue;
            }

            int middle = line.get(line.size() / 2);
            correct_orders += middle;
        }

        System.out.println("Part 1: " + correct_orders);

        endTime = System.nanoTime();
        duration = (endTime - startTime); // in nanoseconds
        durationInSeconds = duration / 1_000_000_000.0;
        System.out.println("Time taken - Part 1: " + durationInSeconds + " seconds");

        // Part 2
        startTime = System.nanoTime();

        correct_orders = 0;
        for (int i = 0; i < invalid_ordered.size(); i++) {
            ArrayList<Integer> line = order.get(invalid_ordered.get(i));
            for (int j = 0; j < line.size(); j++) {
                int current = line.get(j);
                if (!ruleMap.containsKey(current)) {
                    continue;
                }
                for (int k = 0; k < j; k++) {
                    boolean inserted = false;

                    for (Integer rule : ruleMap.get(current)) {
                        if (rule == line.get(k)) {
                            line.add(k, current);
                            line.remove(j + 1);
                            inserted = true;
                            break;
                        }
                    }

                    if (inserted) {
                        break;
                    }
                }
            }

            int middle = line.get(line.size() / 2);
            correct_orders += middle;
        }

        System.out.println("Part 2: " + correct_orders);

        endTime = System.nanoTime();
        duration = (endTime - startTime); // in nanoseconds
        durationInSeconds = duration / 1_000_000_000.0;
        System.out.println("Time taken - Part 2: " + durationInSeconds + " seconds");
    }

}