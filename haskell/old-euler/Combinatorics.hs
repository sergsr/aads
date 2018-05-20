module Combinatorics where

import qualified Data.List as DL
import qualified Data.Bits as DB

-- | Factorial
fact :: (Integral a) => a -> a
fact n = product [1..n]

-- | Binomial coefficient
choose :: (Integral a) => a -> a -> a
choose n k = product [n - k + 1..n] `quot` fact k

-- | Test if a list of numbers is a winning NIM heap arrangement
isNimWin :: [Integer] -> Bool
isNimWin l = 0 /= DL.foldl' DB.xor 0 l

