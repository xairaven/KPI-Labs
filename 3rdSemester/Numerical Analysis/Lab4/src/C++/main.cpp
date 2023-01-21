#include <iostream>
#include <cmath>
#include <iomanip>

using namespace std;

class Polynom {
public:
	double coef;
	int degree;
	Polynom* next;

	Polynom()
		: coef(0.0), degree(0), next(nullptr) {
	}
	Polynom(double _coef, int _degree, Polynom* _next)
		: coef(_coef), degree(_degree), next(_next) {
	}

	Polynom product(Polynom c) {
		Polynom product;
		Polynom* ptr = &product;
		Polynom* a = this;
		while (a) {
			Polynom* b = &c;
			while (b) {
				*ptr = Polynom(a->coef * b->coef, a->degree + b->degree, nullptr);
				ptr->next = new Polynom();
				ptr = ptr->next;
				b = b->next;
			}
			a = a->next;
		}
		return product;
	}
	void deleteZeroCoef() {
		Polynom* a = this;
		while (a) {
			if (a->next) {
				if (a->next->coef == 0) {
					Polynom* temp = a->next;
					a->next = temp->next;
					delete temp;
				} else a = a->next;
			} else a = a->next;
		}
	}
	void simplify() {
		Polynom* a = this;
		while (a) {
			Polynom* b = a->next;
			while (b) {
				if (b->degree == a->degree) {
					a->coef += b->coef;
					b->coef = 0;
				}
				b = b->next;
			}
			a = a->next;
		}
		this->deleteZeroCoef();
	}
	double calculate(double x) {
		double rez = 0;
		Polynom* p = this;
		while (p) {
			rez += p->coef * pow(x, p->degree);
			p = p->next;
		}
		return rez;
	}

	void print() {
		if (coef != 0) {
			if (coef < 0) cout << "-";
			if (fabs(coef) != 1) cout << fabs(coef);
			else if (fabs(coef) == 1 && degree == 0) cout << fabs(coef);
			if (degree != 0) cout << "x";
			if (degree > 1) cout << "^" << degree;
		}
		if (next) {
			if (next->coef > 0) cout << " + ";
			else cout << " ";
			next->print();
		}
	}
};

double dividedDifference(double*& x, double*& y, int i, int k) {
	if (i == k) return y[i];
	return ((dividedDifference(x, y, i, k - 1) - dividedDifference(x, y, i + 1, k)) / (x[i] - x[k]));
}
double calculateNewtonsPolynom(double x, double*& dd, Polynom*& pn, int n) {
	double rez = dd[0];
	for (int i = 1; i < n - 1; i++) {
		if (dd[i] != 0) rez += dd[i] * pn[i - 1].calculate(x);
	}
	return rez;
}
void calculateNewtonsPolynomError(double*& x, double*& dd, Polynom*& pn, int n) {
	double step = (x[1] - x[0]) / 5;
	for (int i = 0; i < n * 4.2; i++) {
		double xi = x[0] + step * i;
		cout << " x = " << setprecision(2) << xi << endl;
		double y = sin(xi) + cbrt(2 * xi);
		cout << " y(x) = " << setprecision(10) << y << endl;
		double p = calculateNewtonsPolynom(xi, dd, pn, n);
		cout << " P(x) = " << p << endl;
		cout << " Error = " << fabs(p - y) << endl << endl;
	}
}
void newtonsPolynom(double*& x, double*& y, int n) {
	double* dd = new double[n];
	for (int i = 0; i < n; i++) {
		dd[i] = dividedDifference(x, y, 0, i);
	}

	Polynom* pn = new Polynom[n - 1];
	for (int i = 0; i < n - 1; i++) {
		pn[i] = Polynom(1, 1, nullptr);
		pn[i].next = new Polynom((-1) * x[i], 0, nullptr);
		if (i > 0) {
			pn[i] = pn[i].product(pn[i - 1]);
			pn[i].simplify();
		}
	}

	cout << " Newton's polynom:" << endl;
	for (int i = 0; i < n; i++) {
		if (i == 0) cout << " " << dd[i];
		else if (dd[i] != 0) {
			if (dd[i] > 0) cout << " + ";
			else cout << " - ";
			cout << "(";
			pn[i - 1].print();
			cout << ") * " << fabs(dd[i]);
		}
	}
	cout << endl << endl;

	calculateNewtonsPolynomError(x, dd, pn, n);
}

void calculateACoefs(double*& a, double**& system, double*& y, int n) {
	a[0] = 0.0;
	for (int i = 1; i < n - 2; i++) {
		a[i] = ((-1.0) * system[i - 1][2]) / (system[i - 1][0] * a[i - 1] + system[i - 1][1]);
	}
}
void calculateBCoefs(double*& b, double*& a, double**& system, double*& y, int n) {
	b[0] = 0.0;
	for (int i = 1; i < n - 2; i++) {
		b[i] = (system[i - 1][3] - system[i - 1][0] * b[i - 1]) / (system[i - 1][0] * a[i - 1] + system[i - 1][1]);
	}
}
double calculateCSpline(double xi, double*& a, double*& b, double*& c, double*& d, double*& x0, int n) {
	double x = 0, ai = 0, bi = 0, ci = 0, di = 0;

	for (int i = 1; i < n; i++) {
		x = xi - x0[i - 1];
		ai = a[i - 1];
		bi = b[i - 1];
		ci = c[i - 1];
		di = d[i - 1];
		if (xi < x0[i]) break;
	}
	double rez = ai + bi * x + ci * pow(x, 2) + di * pow(x, 3);
	return rez;
}
void calculateCSplinePolynomError(double*& x, double*& a, double*& b, double*& c, double*& d, int n) {
	double step = (x[1] - x[0]) / 5;
	for (int i = 0; i < n * 4.2; i++) {
		double xi = x[0] + step * i;
		cout << " x = " << setprecision(2) << xi << endl;
		double y = sin(xi) + cbrt(2 * xi);
		cout << " y(x) = " << setprecision(10) << y << endl;
		double p = calculateCSpline(xi, a, b, c, d, x, n);
		cout << " S(x) = " << p << endl;
		cout << " Error = " << fabs(p - y) << endl << endl;
	}
}
void cSpline(double*& x, double*& y, int n) {
	double* a = new double[n - 1];
	double* b = new double[n - 1];
	double* c = new double[n - 1];
	double* d = new double[n - 1];
	double* h = new double[n - 1];

	for (int i = 0; i < n - 1; i++) {
		h[i] = x[i + 1] - x[i];
		if (i == 0 || i == n - 2) c[i] = 0.0;
	}

	double** system = new double* [n - 3];
	for (int i = 0; i < n - 3; i++) {
		system[i] = new double[4];
		for (int j = 0; j < 4; j++) {
			switch (j) {
			case 0: system[i][j] = h[i + 1]; break;
			case 1: system[i][j] = 2 * (h[i + 1] + h[i + 2]); break;
			case 2: system[i][j] = h[i + 2]; break;
			case 3: system[i][j] = 3 * (((y[i + 2] - y[i + 1]) / h[i + 2]) - ((y[i + 1] - y[i]) / h[i + 1])); break;
			}
		}
	}
	calculateACoefs(a, system, y, n);
	calculateBCoefs(b, a, system, y, n);
	for (int i = 0; i < n - 3; i++) delete[] system[i];
	delete[] system;
	for (int i = n - 3; i > 0; i--) c[i] = a[i] * c[i + 1] + b[i];

	for (int i = 0; i < n - 1; i++) {
		a[i] = y[i];
		if (i == n - 2) {
			d[i] = ((-1.0) * c[i]) / (3 * h[i]);
			b[i] = ((y[i + 1] - y[i]) / h[i]) - ((2 * h[i] * c[i]) / 3);
		} else {
			d[i] = (c[i + 1] - c[i]) / (3 * h[i]);
			b[i] = ((y[i + 1] - y[i]) / h[i]) - ((h[i] * (c[i + 1] + 2 * c[i])) / 3);
		}
		cout << " S" << i + 1 << ":" << endl << setprecision(15) << " a = " << setw(18) << a[i] << "\tb = " << setw(18) << b[i] << endl << " c = " << setw(18) << c[i] << "\td = " << setw(18) << d[i] << endl << endl;
	}
	calculateCSplinePolynomError(x, a, b, c, d, n);

	cout << endl;
}

int main() {
	cout << endl << " Function: y(x) = sin(x) + (2*x)^(1/3)" << endl;
	cout << " Interpolation nodes: -4, -2, 0, 2, 4" << endl << endl;

	int n = 5;
	double* x = new double[n];
	double* y = new double[n];
	double step = -4.0;
	for (int i = 0; i < n; i++) {
		x[i] = step;
		step += 2;
		y[i] = sin(x[i]) + cbrt(2 * x[i]);
	}
	newtonsPolynom(x, y, n);
	cSpline(x, y, n);

	return 0;
}

