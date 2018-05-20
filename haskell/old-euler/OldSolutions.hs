module OldSolutions where
import qualified Data.Char     as DC
import qualified Data.List     as DL
import qualified Data.Sequence as DS
import qualified Combinatorics as C
import qualified NumberTheory  as NT
import qualified Sums          as S

-- * #1-10

-- TODO: maybe find a prettier way to rewrite this using rotations and chunking
-- ^ if performance isn't too much worse i guess?
-- | Maximum product of <1st arg> consecutive digits in <2nd arg>.
pe8 :: Int -> Integer -> Integer
pe8 n xs = fst (DL.foldl' f (product firstN, firstN) xs')
           where xs'       = NT.toDigs xs
                 firstN    = reverse $ take n xs'
                 f (a,b) x = ( max a $ product section, section )
                             where section = x : init b

-- TODO: too slow to execute in REPL. read the pdf, and don't brute force
-- | First Pythagorean triplet whose terms add up to input.
pe9 :: (Integral a) => a -> a
pe9 n = head [ a*b*c | a <- l, b <- l, c <- l,
                       a < b,
                       b < c,
                       a^2 + b^2 == c^2,
                       a + b + c == n
             ]
        where l = [1..n]

-- * #11-20

-- TODO: clean this up, use some nice data.list methods
-- too slow for REPL
-- | Tells which number in the input gives the longest Collatz sequence
pe14 :: [Int] -> Int
pe14 = fst . DL.foldl' f (1, 1)
       where f a x | snd a < NT.collLen x = (x, NT.collLen x)
                   | otherwise            = a

-- | Sum of all amicable numbers < input.
pe21 :: Integer -> Integer
pe21 n = sum [ xds x | x <- [1..n-1], xds x /= x, xds(xds x) == x ]
         where xds = sum . NT.divs

-- | <2nd arg>'th lexicographic permutation of <1st arg>
pe24 :: [Int] -> Int -> Int
pe24 xs n = NT.fromDigs $ DL.sort (DL.permutations xs) !! n

-- | First fibonacci term to contain <input> digits
pe25 n = fst . head . dropWhile toosmall . zip [1..] $ map NT.fib [1..]
         where toosmall (_,x) = NT.logBy 10 x < n

-- | Count of unique numbers a^b for a, b from 2 to <input>
pe29 :: Integer -> Int
pe29 n = length $ DL.group $ DL.sort [ a^b | a <- [2..n], b <- [2..n] ]

-- * #41-50

-- | Product of digits of Champernowne constant. Input determines which digits.
pe40 :: [Int] -> Int
pe40 ds = product [ xs !! x | x <- ds ]
          where xs = concatMap NT.toDigs [0..]

-- * #51-60

pe53 :: Integer -> Integer -> Int
pe53 a b = length $ filter (>b) [ C.choose x y | x <- [1..a], y <- [1..a] ]

-- * #61-70

pe92 :: Int -> Int
pe92 ub = length $ filter test [1..ub]
          where test 89 = True
                test 1  = False
                test x  = test $ sum $ map (^2) $ NT.toDigs x
