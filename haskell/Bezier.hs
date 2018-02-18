-- TODO: make some kind of bounded float type / look into smart ctors for t;
-- params don't need to go over 1. Maybe create a normalized version with
-- adjacent max?
-- | Module implementing Bezier curves (just the math)
module Bezier where
import Data.Complex

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

