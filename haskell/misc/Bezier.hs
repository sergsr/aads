-- TODO: make some kind of bounded float type / look into smart ctors for t;
-- params don't need to go over 1. Maybe create a normalized version with
-- adjacent max?
-- | Module implementing Bezier curves (just the math)
module Main where
import Data.Complex
import Data.List.Split
import System.Environment (getArgs)

type Curve = Float -> Complex Float

-- | Straight-line homotopy from first curve to second
slh :: Curve -> Curve -> Curve
slh c1 c2 t = t1 * c1 t + t2 * c2 t
  where t1 = (1-t)    :+ 0
        t2 = t        :+ 0

-- | Takes a list of n 2D points and returns an nth-degree Bezier curve
-- recursively
recBez :: [Complex Float] -> Curve
recBez [p] = const p
recBez ps  = slh (recBez $ init ps) (recBez $ tail ps)

-- | Reads list of float literals as rectangular coords for complex numbers
readPts :: [String] -> [Complex Float]
readPts ps = zipWith (:+) (map head pairs) (map last pairs)
  where pairs = chunksOf 2 $ map read ps

-- | Turns list of control points for a Bezier curve into its graph
genPts :: [Complex Float]-> [Complex Float]
genPts ss = map (recBez ss) [0.00,0.01..1.00]

-- | Stringifies list of complex numbers as space delimited rectangular form
printPts :: [Complex Float] -> [String]
printPts = map (\z -> show (realPart z) ++ ' ' : show (imagPart z))

main = getArgs >>= mapM_ putStrLn . printPts . genPts . readPts
