#include <iostream>
using namespace std;

enum S
{
	a = 0,
	b,
	c,
	d,
	e,
	f,
	g
};
int main()
{
	union Data{
		struct {
			int x;
			int y;
		}s;
		int x;
		int y;
	}d;
	d.x = 1;
	d.y = 2;
	d.s.x = d.x * d.x;
	d.s.y = d.y * d.y;
	cout << d.s.x << " " << d.s.y << endl;
	cout << S::a << endl;
	cout << S::b << endl;
	cout << S::c << endl;
	cout << S::d << endl;
	cout << S::e << endl;
	cout << S::f << endl;
	cout << S::g << endl;
	return 0;
}
