namespace Lab3;

public static class Calculations
{
	public static void Bisection(NonLinearEquation equation, double a, double b, double precision)
	{
		double fa = Function(equation, a), fb = Function(equation, b);
		double c, fc;
		do {
			c = (a + b) / 2.0;
			fc = Function(equation, c);
			if (fa * fc < 0) {
				b = c;
				fb = fc;
			} else if (fc * fb < 0) {
				a = c;
				fa = fc;
			}
		} while (Math.Abs(b - a) > precision && Math.Abs(fc) > precision);

		Console.WriteLine($"Bisection: {c}");
	}

	public static void Chord(NonLinearEquation? equation, double a, double b, double precision)
	{
		var fa = Function(equation, a);
		var fb = Function(equation, b);
		double x = a, fx, buf;
		
		do {
			buf = x;
			x = (a * fb - b * fa) / (fb - fa);
			fx = Function(equation, x);
			if (fa * fx < 0) {
				b = x;
				fb = fx;
			} else if (fx * fb < 0) {
				a = x;
				fa = fx;
			}
		} while (Math.Abs(x - buf) > precision && Math.Abs(fx) > precision);

		Console.WriteLine($"Chord: {x}");
	}

	public static void Newtons(NonLinearEquation equation, double a, double b, double precision)
	{
		NonLinearEquation? derivative = null;
		Derivative(ref derivative, equation);

		var fa = Function(equation, a);
		var fb = Function(equation, b);
		double x = a, fx = fa, buf;

		do {
			buf = x;
			
			double dx = DerivativeValue(derivative, x);
			
			x -= (fx / dx);
			fx = Function(equation, x);
			if (fa * fx < 0) {
				fb = fx;
			}
			else if ((fx * fb) < 0) {
				fa = fx;
			}
		} while (Math.Abs(x - buf) > precision && Math.Abs(fx) > precision);

		Console.WriteLine($"Newtons: {x}");
	}
	
	private static double Function(NonLinearEquation? equation, double x)
	{
		var item = equation;
		double f = 0;
		
		while (item != null)
		{
			f += (item.Coef * Math.Pow(x, item.Degree));
			item = item.Next;
		}
		
		return f;
	}

	private static void Derivative(ref NonLinearEquation? derivative, NonLinearEquation? equation)
	{
		var item = equation;
		while (item != null)
		{
			var coef = item.Coef * item.Degree;
			if (coef != 0)
			{
				derivative = new NonLinearEquation(coef, item.Degree - 1, derivative);
			}

			item = item.Next;
		}
	}

	private static double DerivativeValue(NonLinearEquation? derivative, double x)
	{
		var item = derivative;
		double f = 0;

		while (item != null)
		{
			f += (item.Coef * Math.Pow(x, item.Degree));
			item = item.Next;
		}

		return f;
	}
}