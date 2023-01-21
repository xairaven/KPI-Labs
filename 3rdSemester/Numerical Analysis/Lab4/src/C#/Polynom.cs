namespace Lab4;

public class Polynom
{
    public double Coef { get; private set; }
    public int Degree { get; }
    public Polynom? Next { get; set; }

    public Polynom()
    {
        Coef = 0.0;
        Degree = 0;
        Next = null;
    }

    public Polynom(double coef, int degree, Polynom? next)
    {
        Coef = coef;
        Degree = degree;
        Next = next;
    }

    public Polynom? Product(Polynom? c)
    {
        Polynom? product = null;
        Polynom? a = this;

        while (a != null)
        {
            var b = c;
            while (b != null)
            {
                product = new Polynom(a.Coef * b.Coef, a.Degree + b.Degree, null);
                product.Next = new Polynom();
                product = product.Next;
                b = b.Next;
            }

            a = a.Next;
        }

        return product;
    }

    public void DeleteZeroCoef()
    {
        Polynom? a = this;

        while (a != null)
        {
            if (a.Next is { Coef: 0 })
            {
                Polynom? temp = a.Next;
                a.Next = temp.Next;
                temp = null;
            }
            else
            {
                a = a.Next;
            }
        }
    }

    public void Simplify()
    {
        Polynom? a = this;

        while (a != null)
        {
            var b = a.Next;

            while (b != null)
            {
                if (b.Degree == a.Degree)
                {
                    a.Coef += b.Coef;
                    b.Coef = 0;
                }
                
                b = b.Next;
            }

            a = a.Next;
        }
    }

    public double Calculate(double x)
    {
        double result = 0;

        Polynom? p = this;
        while (p != null)
        {
            result += p.Coef * Math.Pow(x, p.Degree);
            p = p.Next;
        }
        
        return result;
    }

    public void Print()
    {
        if (Coef != 0)
        {
            if (Coef < 0) Console.Write(" - ");
            
            if (Math.Abs(Coef) != 1) Console.Write(Math.Abs(Coef));
            else if (Math.Abs(Coef) == 1 && Degree == 0) Console.Write(Math.Abs(Coef));
            
            if (Degree != 0) Console.Write("x");
            if (Degree > 1) Console.Write($"^{Degree}");
        }

        if (Next != null)
        {
            if (Next.Coef > 0) Console.Write(" + ");
            else Console.Write(" ");
            
            Next.Print();
        }
    }
}