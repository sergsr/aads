#include <iostream>
#include <math.h>

// TODO: probably fucking up on overflow

using namespace std;

/* fast integer exponentiation ish; should be O(log(e) * multiplications ) */
unsigned int power( unsigned int n, unsigned int e ){

  unsigned long long multiplier = n;
  unsigned long long result = 1;

  for( ; e > 0; e = e >> 1 ){
    cout << "e " << e << endl;
    if( e & 1 )
      result *= multiplier;
    multiplier *= multiplier;
  }

  return result;
}

int main( int argc, char** argv ){
  cout << pow(23, 17) << endl;
  cout << power(23, 17) << endl;
}

