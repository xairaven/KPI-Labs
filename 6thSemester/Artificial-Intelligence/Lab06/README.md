# Лабораторна робота №6

- Варіант: 19
- Скрипт: [*Файл Prolog (.pl)*](./src/task.pl)
- Оцінка: 6/6

### Завдання
Написати програму рішення вправи, вказаної в індивідуальному завданні:

![](./misc/Example.png)

### Запити

Приклад 1:
```prolog
[debug] 15 ?- process_list([1, 2, 3, 4, 5, 6, 7, 8, 9], Result).
Result = [4, 5, 6, 3, 2, 1, 7, 8, 9].
```

Приклад 2:
```prolog
[debug] 16 ?- process_list([1,2,3], Result). 
Result = [2, 1, 3].
```