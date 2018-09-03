{-# LANGUAGE TemplateHaskell #-}
module Main where

import Control.Arrow ((>>>))

import Data.ByteString.Char8 (unpack)
import Data.FileEmbed (embedFile)

import Lib

solution :: (Show a) => Int -> a -> IO ()
solution n a = putStrLn $ "Problem " ++ show n ++ " solution: " ++ show a

main :: IO ()
main = do
  solution  1 $ multiplesOf3And5 $ 10 ^ 3
  solution  2 $ evenFibonacciNumbers $ 4 * 10 ^ 6
  solution  3 $ largestPrimeFactor 600851475143
  solution  5 $ smallestMultiple 20
  solution  6 $ sumSquareDifference 100
  let input013 = map read $ lines $ unpack $(embedFile "data/input013.txt")
  solution 13 $ largeSum input013
  solution 15 $ latticePaths 20
  solution 16 $ powerDigitSum $ 2 ^ (10 ^ 3)
  let input018 = map ((map read) . words) $ lines $ unpack $(embedFile "data/input018.txt")
  solution 18 $ maximumPathSumI input018
  solution 20 $ factorialDigitSum 100
  let input067 = map ((map read) . words) $ lines $ unpack $(embedFile "data/input067.txt")
  solution 67 $ maximumPathSumII input067
