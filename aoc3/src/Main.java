import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.stream.Stream;

public class Main {

    public static void main(String[] args) throws IOException {

        System.out.println(new File(".").getAbsoluteFile());
        int pos = 0;
        int lineNum = 0;
        int[] nums = new int[5];
        int[] nums_zero = new int[5];

        Arrays.fill(nums, 0);
        try(BufferedReader br = new BufferedReader(new FileReader("test.txt"))) {
            for(String line; (line = br.readLine()) != null; ) {
                System.out.print(line.charAt(pos));

                // process the line.
                if (line.charAt(pos) == '1') {
                    nums[pos] += 1;
                } else {
                    nums_zero[pos] += 1;
                }
                pos++;
                if (pos == 4) {
                    pos = 0;
                }

            }
            // line is not visible here.
        } catch (Exception e) {
            e.printStackTrace();
        }
        for (int i = 0; i < nums.length; i++) {
            System.out.println(nums[i]);

        }
        // 0 0 1 1 0 0 1 1 1 0 1 0 --> 4
        // 1 1 0 0 1 1 0 0 0 1 0 1  --> 25


    }
    
}