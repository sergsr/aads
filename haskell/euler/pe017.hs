import System.Environment (getArgs)

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

main = getArgs >>= print . pe017 . read . head
