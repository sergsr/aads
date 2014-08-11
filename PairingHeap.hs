-- | Pairing heap implementation of a priority queue
-- TODO: modernize for GHC 7.10 - might just mean modifying these pragmas / imports,
-- maybe change fold name in pairing?
{-# LANGUAGE DeriveFunctor, DeriveFoldable #-}
module PairingHeap where
import Data.Functor
import Data.Monoid
-- import Data.Foldable
import Data.List (unfoldr)

data PairingHeap k = Nil
                   | Heap k [PairingHeap k]
                   deriving (Show,Functor{-,Foldable-})

-- | Mappend corresponds to the join of two heaps (forgetful functor maps to multiset union)
instance (Ord k) => Monoid (PairingHeap k) where
  mempty  = Nil
  Nil `mappend` h   = h
  h   `mappend` Nil = h
  x@(Heap xk xs) `mappend` y@(Heap yk ys) | xk < yk   = Heap xk (y:xs)
                                          | otherwise = Heap yk (x:ys)

-- | Create heap with a single key
single :: k -> PairingHeap k
single k = Heap k []

-- | Insert a key into the heap
insert :: (Ord k) => PairingHeap k -> k -> PairingHeap k
insert h k = h <> single k

-- | Construct from a list
fromList :: (Ord k) => [k] -> PairingHeap k
fromList = mconcat . fmap single

-- | Return the minimum key, or Nothing if empty
getMin :: PairingHeap k -> Maybe k
getMin Nil        = Nothing
getMin (Heap k _) = Just k

-- | Remove minimum key from heap. This is where the name "pairing" comes from
deleteMin :: (Ord k) => PairingHeap k -> PairingHeap k
deleteMin Nil = Nil
deleteMin (Heap _ hs) = mconcat $ pair hs
   where pair (a:b:t) = (a <> b) : pair t
         pair hs      = hs

-- | (Min key of heap, heap with min key removed)
getAndDelMin :: (Ord k) => PairingHeap k -> Maybe (k, PairingHeap k)
getAndDelMin h = getMin h >>= (\x -> Just (x, deleteMin h))

-- | Get a list of all keys in acsending order
ascendingKeys :: (Ord k) => PairingHeap k -> [k]
ascendingKeys = unfoldr getAndDelMin

-- | Pairing heap sort
phSort :: (Ord k) => [k] -> [k]
phSort = ascendingKeys . fromList
