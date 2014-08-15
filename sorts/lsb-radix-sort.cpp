#include <iostream>

// List-ish LSByte radix sort, AOCP Vol. 3 pg. 171
void algoR( unsigned intdeq &seq ){

  unsigned int byte, i, x;

  const unsigned int total_bytes = sizeof( unsigned int );
  unsigned int mask = 255;

  std::deque< unsigned int >* buckets[255];
  for( i = 0; i < 255; ++i )
    buckets[i] = new std::deque< unsigned int >();

  for( byte = 0; byte < total_bytes; ++byte ){
    // stick all the sequence elements into buckets, in FIFO order
    while( !seq.empty() ){
      x = seq.front();
      seq.pop_front();
      buckets[ (x & mask) >> 8*byte ]->push_back(x);
    }
    // set sequence equal to the concatenation of the buckets
    for( unsigned intdeq* b : buckets ){
      seq.insert( seq.end(), b->begin(), b->end() );
      b->clear();
    }
    // move bitmask to the next byte
    mask = mask << 8;
  }

  for( auto b : buckets )
    delete b;
}
