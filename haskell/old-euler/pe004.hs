import System.Environment (getArgs)
import NumberTheory (toDigs)

-- TODO: use log 10 and check palindrome with division and mod
-- see if there's a good bit hack for this

pe4 :: Int -> Int
pe4 n = maximum [ x*y | x <- dom, y <- dom, x <= y,
                     toDigs (x*y) == reverse (toDigs (x*y)) ]
  where dom = [10^n - 1, 10^n - 2 ..10^(n-1)]

main = getArgs >>= print . pe4 . read . head
