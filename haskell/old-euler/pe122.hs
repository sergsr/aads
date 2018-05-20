import Prelude --for haddock
import Control.Monad (forever)
import NumberTheory (logBy)

-- | Too long to describe in a sentence
pe122 :: Int -> Int
pe122 n = sum $ map minMults [1..n]

-- | this does binary method, which is apparently not the most efficient
minMults n = g n 0
  where g 1 a = a
        g n a | odd n     = g (n-1)        (a+1)
              | otherwise = g (n `quot` 2) (a+1)

-- try a quadratic dynamic programming solution - 1 : 1 : min $ zipWith (+) shit
main = forever $ readLn >>= (print . pe122)
