module NumberTheory where

import qualified Data.Bits as DB
import qualified Data.List as DL

-- * General

-- | Count how many times a function can iterate - seems like a common pattern..
countUntil :: (Integral b) => (a -> Bool) -> (a -> a) -> a -> b
countUntil test f x = snd $ until (test . fst) applyAndInc (x, 0)
                      where applyAndInc (a,b) = (f a, b + 1)
-- TODO: I feel like there's something in the base libs for this?

-- * Base-dependent

-- | Integer logarithm with arbitrary base
logBy :: Int -> Int -> Int
logBy base = countUntil (<base) (`quot` base)

-- * Divisibility related

-- | Tests to see if first argument has the second as a factor
hasDiv :: (Integral a) => a -> a -> Bool
hasDiv a b = a `mod` b == 0

-- | What's left after iteratively dividing the first number by the second
leftover :: (Integral a) => a -> a -> a
leftover x y = until (not . (`hasDiv` y)) (`quot` y) x

-- | How many times the first number can divide the second
multiplicity :: (Integral a) => a -> a -> a
multiplicity x = countUntil (not . (`hasDiv` x)) (`quot` x)

-- | Primality test
isPrime :: (Integral a) => a -> Bool
isPrime = (1==) . length . pDivs

-- | List of prime divisors of a number
pDivs :: (Integral a) => a -> [a]
pDivs n = DL.unfoldr f (abs n, 2)
          where f (a, b) | a < 2 = Nothing
                         | a `hasDiv` b = Just (b, (a `leftover` b, b + 1 ))
                         | otherwise    = f (a, b + 1)

-- | List of (multipicity, prime divisor)'s of a number
pDivMults :: (Integral a) => a -> [(a,a)]
pDivMults n = map (\x -> (multiplicity x n, x)) $ pDivs n

-- | How many divisors a number has (including 1 and itself)
divsCount :: (Integral a) => a -> a
divsCount = product . map ((+1) . fst) . pDivMults

-- | List of all divisors - REALLY REALLY SLOW
divs :: (Integral a) => a -> [a]
divs n = filter (hasDiv $ abs n) [1..abs n `quot` 2] ++ [n]
-- TODO: make this not brute force...

-- | Really stupid sievy method to get all primes up to an upper bound
bPrimes ub = pDivs $ vlcm [2..ub]

-- | Euler's totient function
eutot :: Integer -> Integer
eutot n = n * product ds' `quot` product ds
          where ds  = pDivs n
                ds' = map (subtract 1) ds

-- * Sequences and special cases

-- ** Collatz

-- | Generate the next number in a Collatz sequence.
collStep :: (Integral a) => a -> a
collStep n | even n    = n `quot` 2
           | otherwise = 3*n + 1

-- | Determine number of steps required to reach 1 in a Collatz sequence
-- starting with n.
collLen :: Int -> Int
collLen = (+1) . countUntil (==1) collStep
