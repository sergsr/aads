module Main where

import Lib
import Paths_euler (getDataFileName)
-- https://www.haskell.org/cabal/users-guide/developing-packages.html#accessing-data-files-from-package-code

solution :: (Show a) => Int -> a -> IO ()
solution n a = putStrLn $ "Problem " ++ show n ++ " solution: " ++ show a

main :: IO ()
main = do
  solution  1 $ multiplesOf3And5 $ 10 ^ 3
  solution  2 $ evenFibonacciNumbers $ 4 * 10 ^ 6
  solution  3 $ largestPrimeFactor 600851475143
  solution  5 $ smallestMultiple 20
  solution  6 $ sumSquareDifference 100
  --solution 13 $ largeSum -- TODO: this needs to read a large input. getDataFileName
  solution 15 $ latticePaths 20
  solution 16 $ powerDigitSum $ 2 ^ (10 ^ 3)
  --solution 18 $ maximumPathSumI -- TODO: this requires special input
  solution 20 $ factorialDigitSum 100
