package Lab3;
import Lab3.structs.DoublyLinkedList;
import Lab3.structs.List;
import Lab3.structs.Writer;

import java.io.FileNotFoundException;
import java.util.Scanner;

/**
 * Лабораторна робота №3
 * 1. Дослідити особливості створення однонаправлених і двонаправлених списків.
 * 2. Вивчити і реалізувати механізми додавання нових записів у список,
 * пошуку записів у списку за певними полями, видалення записів зі списку та
 * редагування знайдених записів, а також збереження всього списку у файлі та зчитування списку
 * із файлу до пам’яті з відновленням всіх зв’язків.
 * 3. Розробити Блок-схему програмного алгоритму.
 * 4. Оформити ЗВІТ до лабораторної роботи згідно вимог та методичних рекомендацій.
 *
 * РЕЗУЛЬТАТ РОБОТИ ПОТРІБНО:
 * 1. Роздрукувати (вивести на екран) попередньо сформовані та підготовлені для запису в файл дані.
 * 2. Роздрукувати (вивести на екран) результат виконання операції читання даних із файлу.
 * 3. Відкритий для редагування програмний код розмістити на сайті Repl (посилання через кнопку «+ Invite»).
 * 4. ЗВІТ до комп’ютерного практикуму № 3 для перевірки додати в свій Клас на ресурсі Classroom.
 *
 * Виконав студент групи ТР-12
 * Ковальов Олександр
 * Номер варіанту: 22
 */
public class Main {
    public static void main(String[] args) throws FileNotFoundException {
        System.out.println("Hello, user! There are three variants of input:");
        System.out.println("1. Default values");
        System.out.println("2. Standard input");
        System.out.println("3. From file (path must be in args)\n");

        var scan = new Scanner(System.in);
        int choice = scan.nextInt();

        var list = new List();
        var doublyList = new DoublyLinkedList();

        switch (choice) {
            case 1 -> {
                for (int i = 1; i < 5; i++) {
                    list.push(new Hairdressers("Label" + i, "Address" + i,
                            i * 10, i + "-" + i * 3));

                    doublyList.pushLast(new Hairdressers("Label" + i, "Address" + i,
                            i * 10, i + "-" + i * 3));
                }
                System.out.println("\nDone! Pushed default values\n");
            }
            case 2 -> {
                System.out.println("Number of objects:");
                int amount = scan.nextInt();

                for (int i = 0; i < amount; i++) {
                    System.out.print("Label: ");
                    String label = scan.next();
                    System.out.print("Address: ");
                    String address = scan.next();
                    System.out.print("Number of workers: ");
                    int workers = Integer.parseInt(scan.next());
                    System.out.print("Work time: ");
                    String workTime = scan.next();

                    list.push(new Hairdressers(label, address, workers, workTime));
                    doublyList.pushLast(new Hairdressers(label, address, workers, workTime));
                    System.out.println();
                }
                System.out.println("Done! Pushed values from keyboard\n");
            }
            case 3 -> {
                if (args.length != 1) {
                    throw new FileNotFoundException("File not found");
                }
                list = Hairdressers.readFromFile(args[0]);
                System.out.println("\nDone! Pushed values from file!\n");
            }
            default -> {
                System.out.println("\nInput number 1, 2 or 3");
            }
        }

        System.out.println("List:");
        for(var item : list) {
            System.out.println(item);
        }

        System.out.println("\nWriting all structs in src/Lab3/res/Hairdressers.txt....");
        var writer = new Writer("src/Lab3/res/Hairdressers.txt");
        for (var item : list) {
            writer.printline(item.toString());
        }
        System.out.println("Done!");

        System.out.println("\nSearch element with label \"Label 2\"..");
        Hairdressers example = list.searchByLabel("Label2");
        System.out.println(example);

        System.out.println("\nDeleting it..");
        list.delete(example);
        System.out.println("Done!");

        System.out.println("\nList:");
        for(var item : list) {
            System.out.println(item);
        }
    }
}
