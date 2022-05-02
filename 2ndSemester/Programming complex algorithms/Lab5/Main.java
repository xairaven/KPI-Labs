package Lab5;

public class Main {
    public static void main(String[] args) {
        System.out.println("Kovalyov O., TR-12, var. 21\n");
        task1();
        task2();
        task3();
        System.out.println("\n\n------ TESTING STRUCTURES ------");
        SinglyLinkedList.tests();
        DoublyLinkedList.tests();
        Stack.tests();
        Queue.tests();
    }

    public static void task1() {
        System.out.println("------ TASK 1 ------");
        Stack<Integer> stack = new Stack<>();
        for (int i = 0; i < 10; i++) {
            int rand = (int) (Math.random() * (10 - (-10)) - 10);
            stack.push(rand);
        }
        System.out.println("Stack: ");
        System.out.println(stack);
        System.out.println("Stack peek: " + stack.peek());
        if (stack.peek() < 0) System.out.println("Stack peek < 0");
        else {
            System.out.println("Stack peek >= 0. Deleting 5 elements");
            for (int i = 0; i < 5; i++) {
                System.out.println("Deleting element " + stack.pop());
            }
            System.out.println("Result stack:");
            System.out.println(stack);
        }
        System.out.println();
    }

    public static void task2(){
        System.out.println("------ TASK 2 ------");
        SinglyLinkedList<Integer> list = new SinglyLinkedList<>();
        for(int i = 1; i <= 10; i++) {
            list.push(i);
        }
        int key = 5;
        System.out.println("Key is: " + key);
        System.out.println("Initial list:\n" + list);
        int index = 0;
        for (int i : list) {
            if (i == key) {
                list.task(index);
                break;
            } else {
                index++;
            }
        }
        System.out.println("Result list:\n" + list);
        System.out.println();
    }

    public static void task3() {
        System.out.println("------ TASK 3 ------");
        DoublyLinkedList<Integer> list = new DoublyLinkedList<>();
        for(int i = 1; i <= 10; i++) {
            list.pushLast(i);
        }
        int key = 5;
        System.out.println("Key is: " + key);
        System.out.println("Initial list:\n" + list);
        int index = 0;
        for (int i : list) {
            if (i == key) {
                list.task(index);
                break;
            } else {
                index++;
            }
        }
        System.out.println("Result list:\n" + list);
        System.out.println();
    }
}
