#include <stdio.h>

int sum (int a,int b)
{
	int c = a + b;
	return c;
}

int main()
{
	int x = 5,y = 10,z = 0;
	z = sum(x,y);
	printf("%d\r\n",z);
	return 0;
}