
import java.util.Arrays;
import java.util.Collections;
import java.util.Scanner;

public class main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        
        System.out.println("choose the sorting method (1: alphabet, 2: reversed)");
        String onetwo = sc.nextLine();
        char answer = onetwo.charAt(0);

        System.out.println("enter a list of words or letters: (seperate them with spaces)");
        String input = sc.nextLine();
        String[] words = input.split(" ");

        if (answer == '1') {
            Arrays.sort(words);
            System.out.println(Arrays.toString(words));
        } else if (answer == '2') {
            Arrays.sort(words, Collections.reverseOrder());
            System.out.println(Arrays.toString(words));
        }

        
        
    }

}
