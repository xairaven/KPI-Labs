import java.util.Date; //імпортування пакету для виведення дати
import java.util.Scanner; //імпортування пакету для сканера

class Main {
  //3, варіант 23. Ковальов О., ТР-12
  public static void main(String[] args) {
  	Scanner scan = new Scanner(System.in); //відкриття сканера
  	double Y, D; //оголошення змінних
  	double a = 3.2;
  	double b = 17.5;
  	double x = -4.8;
  	double y = calcY(a,b,x), d = calcD(a,b,x); //повернення значень від методів 
  	System.out.printf("y = %.4f  d = %.4f\n", y, d); //форматоване виведення
  	System.out.printf("\na: "); //отримуємо значення від користувача
  	a = scan.nextDouble();
  	System.out.printf("\nb: ");
  	b = scan.nextDouble();
  	System.out.printf("\nx: ");
  	x = scan.nextDouble();
    y = calcY(a,b,x);
    d = calcD(a,b,x);
  	System.out.printf("y = %.4f  d = %.4f\n", y, d); //ще раз форматоване виведення
  	scan.close(); //закриття потоку 
  	outputDate(); //виведення дати
  }
  public static double calcY(double a, double b, double x) { //знаходження Y
  	double y;
  	y = b*Math.pow(Math.tan(x),2)-a/Math.pow((Math.sin(x/a)),2);
  	return y;
  }

  public static double calcD(double a, double b, double x) { //знаходження D
  	double d;
  	d = a*(Math.pow(Math.E,-1*Math.sqrt(a)))*Math.cos(b*x/a);
  	return d;
  }

  public static void outputDate() { //Дата
  	Date d = new Date();
  	System.out.printf("%tB, %td %tY %tA",d,d,d,d);
  }
}
