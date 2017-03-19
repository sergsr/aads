import System.Environment (getArgs)

pe018 :: [[Int]] -> Int
pe018 = head . foldr combineRows (repeat 0)
  where combineRows t b = zipWith max (zipWith (+) t b) (zipWith (+) t $ tail b)

main = getArgs >>=
  readFile . head >>=
  print . pe018 . map (map read . words) . lines
