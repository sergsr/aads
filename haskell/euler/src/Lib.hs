module Lib
    ( multiplesOf3And5,
      evenFibonacciNumbers,
      largestPrimeFactor,
      smallestMultiple,
      sumSquareDifference,
      largeSum,
      latticePaths,
      powerDigitSum,
      maximumPathSumI,
      factorialDigitSum,
      selfPowers,
      powerfulDigitCounts,
      maximumPathSumII,
      largeNonMersennePrime,
    ) where

import Control.Arrow((>>>))
import Data.Char(digitToInt)

arithmeticSum n | n > 0     = (n * (n + 1)) `quot` 2
                | otherwise = 0

multiplesOf3And5 :: Int -> Int
multiplesOf3And5 n  = sumMults 3 + sumMults 5 - sumMults 15
  where sumMults k = case n `mod` k of
          0 -> k * arithmeticSum (n `quot` k - 1)
          _ -> k * arithmeticSum (n `quot` k)

fibonacci :: [Integer]
fibonacci = 1 : 2 : zipWith (+) fibonacci (drop 1 fibonacci)

evenFibonacciNumbers :: Integer -> Integer
evenFibonacciNumbers n = sum $ takeWhile (< n) $ filter even fibonacci

largestPrimeFactor :: Integer -> Integer
largestPrimeFactor n | abs n < 4 = abs n
largestPrimeFactor n = last $ takeWhile (/= 1) $ scanl f (abs n) [2..]
  where f a x = until (\y -> mod y x /= 0) (`quot` x) a

smallestMultiple :: Int -> Int
smallestMultiple = (!!) (0 : scanl1 lcm [1..])

sumSquareDifference :: Int -> Int
sumSquareDifference n | n < 1     = 0
                      | otherwise = ((n*(n + 1)) `quot` 2)^2 - (n*(n + 1)*(2*n + 1) `quot` 6)

largeSum :: [Integer] -> String
largeSum = sum >>> show >>> take 10

latticePaths :: Integer -> Integer
latticePaths k = product [k + 1 .. 2*k] `quot` product [1..k]

digitSum :: (Num a, Show a, Read a) => a -> a
digitSum = show >>> map (digitToInt >>> fromIntegral) >>> sum

powerDigitSum :: Integer -> Integer
powerDigitSum = digitSum

{--

-- TODO: make this a read-only array?
numberWords :: [String]
numberWords = [
  ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"],
  ["ten", "eleven", "twelve", "thirteen", "fourteen",
    "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"],
  ["twenty", "thirty", "forty",
   "fifty", "sixty", "seventy", "eighty", "ninety"],
  ["hundred"] -- TODO: wat?
]
-- this one turns number -> count.
numberLetters = undefined

-- this one actually gives solution
numberLetterCounts = undefined
-- TODO: make these guys arrays
onesToW :: Int -> Int
onesToW = ([0, 3, 3, 5, 4, 4, 3, 5, 5, 4] !!)

teenToW :: Int -> Int
teenToW = ([3, 6, 6, 8, 8, 7, 7, 9, 8, 8] !!) . (`mod` 10)

tensToW :: Int -> Int
tensToW n = case n `quot` 10 of
  0 ->  onesToW (n `mod` 10)
  1 ->  teenToW n
  x ->  [6, 6, 5, 5, 5, 7, 6, 6] !! (x - 2) + onesToW (n `mod` 10)

hunsToW :: Int -> Int
hunsToW n = case n `quot` 100 of
  0 ->  tensToW (n `mod` 100)
  _ ->  onesToW (n `quot` 100) + tensToW (n `mod` 100) +
     case n `mod` 100 of
       0 ->  7
       _ ->  10

thouToW :: Int -> Int
thouToW n = case n `quot` 1000 of
              0 ->  hunsToW (n `mod` 1000)
              x ->  onesToW x + 8 + hunsToW (n `mod` 1000)

pe017 :: Int -> Int
pe017 n = sum $ map thouToW [1..n]
--}

maximumPathSumI :: [[Int]] -> Int
maximumPathSumI = head . foldr combineRows (repeat 0)
  where combineRows t b = zipWith max (zipWith (+) t b) (zipWith (+) t $ tail b)

factorialDigitSum :: Integer -> Integer
factorialDigitSum n = digitSum $ product [1..n]

{--
namesScores = undefined
split :: String -> [String]
split = words . map replace . filter (/= '"')
  where replace ',' = ' '
        replace x   = x

charScore :: String -> Integer
charScore = sum . map (fromIntegral . offsetOrd)
  where offsetOrd c = ord c - ord 'A' + 1

pe022 :: [String] -> Integer
pe022 = sum . map (\(i, s) -> i * charScore s) . zip [1..] . sort
--}

selfPowers :: Integer -> Integer
selfPowers n = sum (map (\x -> x^x) [1..n]) `mod` 10^10

-- TODO: when compiled with optimization, this fails. figure out why
-- | Number of n-digit (base <input>) integers that are also nth powers
powerfulDigitCounts :: Int -> Int
powerfulDigitCounts b = 1 + sum (map f [2..b])
  where f x = floor (lf b / (lf b - lf x))
        lf  = log . fromIntegral

maximumPathSumII = maximumPathSumI

largeNonMersennePrime :: Integer -> Integer
largeNonMersennePrime = mod (28433 * (2^7830457) + 1) . (10^)
