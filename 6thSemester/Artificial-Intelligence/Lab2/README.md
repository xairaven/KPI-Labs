# Лабораторна робота №2

- Варіант: 19
- Скрипт: [*Файл Prolog (.pl)*](./src/Knowledge-Base.pl)
- Графічна схема: [*.png file*](./misc/Scheme.png)
- Оцінка: 6/6

### Завдання:
Описати на SWI Prolog відношення, що визначає споріднений
зв'язок за індивідуальним завданням. У базі даних представити факти, що
представляють одне або декілька з наступних відносин:

```prolog
is_parent(?Parent's name, Children's name).
is_female(?Name).
is_male(?Name).
marriage(?Husband's name, ?Wife's name).
```

Припустимо використовувати лише зазначені структури відношень. Для
представлення заданого відношення використовувати таку структуру
аргументів:

```prolog
is_relation(?Who ?Whom)
```

де `<relation>` відповідає заданому відношенню, `Who` визначає ім'я людини, що
знаходиться у відношенні `<relation>` до людини на ім'я `Whom`. Наприклад,
запит по відношенню «мачуха» (Мері є мачухою Боба):

```prolog
is_stepmother(mary, bob).
```

### Індивідуальне завдання:
Зовиця (сестра дружини).