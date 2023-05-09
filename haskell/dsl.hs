{-# LANGUAGE GADTs #-}
{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE StandaloneDeriving #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE DataKinds #-}
{-# LANGUAGE DeriveAnyClass #-}
{-# LANGUAGE TypeOperators #-}
{-# LANGUAGE FunctionalDependencies #-}
{-# LANGUAGE UndecidableInstances #-}
{-# LANGUAGE AllowAmbiguousTypes #-}
{-# LANGUAGE TypeFamilies #-}
{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE ScopedTypeVariables #-}
{-# LANGUAGE DeriveFunctor #-}

import Prelude hiding (id, (.))
import Data.Functor.Classes (Show1)
import Control.Category
import Data.Bifunctor

-- 2D position
data P2 a = P2 a a deriving (Show, Eq, Ord)

-- Distance value
data Dist a = Dist a deriving (Show, Eq, Ord)

-- Gradient value
data Grad a = Grad a a deriving Show

-- Distance domain
class Distance m i o a | m -> i o a where
    distance :: m -> (i a -> o a)

-- Gradient domain
class Gradient m i o a  | m -> i o a where
    gradient :: m -> (i a -> o a)

-- Translate position modifier
data Translate a = Translate a deriving Show

translate (P2 dx dy) = \(P2 x y) -> P2 (x + dx) (y + dy)

instance (Num a) => Distance (Translate (P2 a)) P2 P2 a where
    distance (Translate p) = translate p

instance (Num a) => Gradient (Translate (P2 a)) P2 P2 a where
    gradient (Translate p) = translate p

-- Point shape
data Point a = Point deriving Show

instance (Floating a) => Distance (Point a) P2 Dist a where
    distance Point = \(P2 x y) -> Dist $ sqrt $ x * x + y * y

instance (Floating a) => Gradient (Point a) P2 Grad a where
    gradient Point = \(P2 x y) -> let l = sqrt $ x * x + y * y in Grad (x / l) (y / l)

-- Isosurface distance modifier
data Isosurface a = Isosurface a deriving (Show, Show1)

instance (Num a) => Distance (Isosurface a) Dist Dist a where
    distance (Isosurface a) = \(Dist b) -> Dist $ b - a
    
instance (Num a) => Gradient (Isosurface a) Grad Grad a where
    gradient _ = id

-- HList implementation
data HList :: [*] -> * where
    HNil :: HList '[]
    HCons :: x -> HList xs -> HList (x ': xs)

(%:) :: x -> HList xs -> HList (x ': xs)
(%:) = HCons
infixr 5 %:

instance Show (HList '[]) where
    show HNil = "HNil"

instance (Show x, Show (HList xs)) => Show (HList (x : xs)) where
    show (HCons x xs) = show x ++ " %: " ++ show xs

instance Eq (HList '[]) where
    HNil == HNil = True

instance (Eq x, Eq (HList xs)) => Eq (HList (x : xs)) where
    (HCons x xs) == (HCons y ys) = x == y && xs == ys

-- Per-element function application
class Apply f a b where
    apply :: f -> a -> b

-- HList map
class MapH f xs ys where
    mapH :: f -> HList xs -> HList ys

instance MapH f '[] '[] where
    mapH _ _ = HNil

instance (Apply f x y, MapH f xs ys) => MapH f (x ': xs) (y ': ys) where
    mapH f (HCons x xs) = apply f x %: mapH f xs

-- HList fold
class FoldrH f acc xs where
    foldrH :: f -> acc -> HList xs -> acc

instance FoldrH f acc '[] where
    foldrH _ acc _ = acc

instance (Apply f x (acc -> acc), FoldrH f acc xs) => FoldrH f acc (x ': xs) where
    foldrH f acc (HCons x xs) = apply f x $ foldrH f acc xs

-- HList chain
class ChainH xs a where
    chainH :: HList xs -> a

instance ChainH '[] (a -> a) where
    chainH _ = id

instance (ChainH xs (b -> c)) => ChainH ((a -> b) ': xs) (a -> c) where
    chainH (HCons x xs) = (chainH xs) . x

-- HList to string
data Pretty = Pretty
instance Show x => Apply Pretty x String where
    apply _ x = show x

-- HList string concat
data Concat = Concat
instance Apply Concat String (String -> String) where
    apply _ x = \y -> x ++ ", " ++ y

-- Reasoning about intermediary types
type family ConstMap (t :: *) (xs :: [*]) :: [*] where
    ConstMap _ '[] = '[]
    ConstMap t (x ': xs) = t ': (ConstMap t xs)

-- HList pretty printer
prettyHList :: forall xs.
               ( MapH   Pretty xs     (ConstMap String xs)
               , FoldrH Concat String (ConstMap String xs)
               ) => HList xs -> String
prettyHList hlist =
  "[" ++ (foldrH Concat "" $ mapH @_ @_ @(ConstMap String xs) Pretty hlist) ++ "]"

-- HList to distance domain
data HDistance = HDistance

instance (Distance m i o a) => Apply HDistance m (i a  -> o a) where
    apply _ x = distance x

-- HList to gradient domain
data HGradient = HGradient

instance (Gradient m i o a) => Apply HGradient m (i a  -> o a) where
    apply _ x = gradient x

-- Lattice join operator
data Join a b = a :\/: b deriving (Show, Functor)

instance Bifunctor Join where
    first f (a :\/: b) = f a :\/: b
    second f (a :\/: b) = a :\/: f b

-- Lattice meet operator
data Meet a b = a :/\: b deriving (Show, Functor)

instance Bifunctor Meet where
    first f (a :/\: b) = f a :/\: b
    second f (a :/\: b) = a :/\: f b

-- Apply a function over a Join via MapH
instance (MapH f a c, MapH f b d) => Apply f (Join (HList a) (HList b)) (Join (HList c) (HList d)) where
    apply f (a :\/: b) = (mapH f a) :\/: (mapH f b)

-- Apply a function over a Meet via MapH
instance (MapH f a c, MapH f b d) => Apply f (Meet (HList a) (HList b)) (Meet (HList c) (HList d)) where
    apply f (a :/\: b) = (mapH f a) :/\: (mapH f b)

-- Single-function bimap wrapper
bmap f j = bimap f f j

-- Combine the contents of a Join via max
runJoin :: Ord b => Join (a -> b) (a -> b) -> a -> b
runJoin (a :\/: b) p = (a p) `max` (b p)

-- Combine the contents of a Meet via min
runMeet :: Ord b => Meet (a -> b) (a -> b) -> a -> b
runMeet (a :/\: b) p = (a p) `min` (b p)

runMeet2 (a :/\: b) (c :/\: d) p =
    let
        a' = a p
        b' = b p
    in
        if a' `min` b' == a'
        then (a', c)
        else (b', d)

--- Concrete test

-- First shape
fieldA :: HList '[Translate (P2 Float), Point Float, Isosurface Float]
fieldA = Translate (P2 2 2) %: Point %: Isosurface 1 %: HNil

-- Second shape
fieldB :: HList '[Translate (P2 Float), Point Float, Isosurface Float]
fieldB = Translate (P2 (-2) (-2)) %: Point %: Isosurface 2 %: HNil

-- First shape in distance domain
fieldDist :: HList '[P2 Float -> P2 Float, P2 Float -> Dist Float, Dist Float -> Dist Float]
fieldDist = mapH HDistance fieldA

-- First shape in gradient domain
fieldGrad :: HList '[P2 Float -> P2 Float, P2 Float -> Grad Float, Grad Float -> Grad Float]
fieldGrad = mapH HGradient fieldA

-- First shape chained into a distance function
dist :: P2 Float -> Dist Float
dist = chainH fieldDist

-- First shape chained into a gradient function
grad :: P2 Float -> Grad Float
grad = chainH fieldGrad

-- Union of first and second shapes
meet ::
    Meet
    (HList '[Translate (P2 Float), Point Float, Isosurface Float])
    (HList '[Translate (P2 Float), Point Float, Isosurface Float])
meet = fieldA :/\: fieldB

-- Union in the distance domain
meet' :: Meet 
    (HList '[P2 Float -> P2 Float, P2 Float -> Dist Float, Dist Float -> Dist Float])
    (HList '[P2 Float -> P2 Float, P2 Float -> Dist Float, Dist Float -> Dist Float])
meet' = apply HDistance meet

-- Union chained into a pair of distance functions
meet'' :: Meet (P2 Float -> Dist Float) (P2 Float -> Dist Float)
meet'' = bmap chainH meet'

-- Combined union evaluation function
meet''' = runMeet2 meet'' meet

-- Input position
pos = P2 0 0

-- 2-tuple of evaluated distance and corresponding shape
meet'''' = meet''' $ pos

-- Corresponding shape in the gradient domain
meet''''' :: HList '[P2 Float -> P2 Float, P2 Float -> Grad Float, Grad Float -> Grad Float]
meet''''' = mapH HGradient (snd meet'''')

-- Corresponding shape chained into a gradient function
meet'''''' :: P2 Float -> Grad Float
meet'''''' = chainH meet'''''

-- Combined position and gradient
out = (fst meet'''', meet'''''' pos)

-- Takeaways:
-- An extensible SDF DSL is composed of:
-- 
-- Symbols representing field functions, I / O modifiers, and higher-order operations
--   These exist to encode polymorphism over multiple field domains,
--   and defer execution to an interpreter system that resolves domain-specific functions at evaluation time
--   A field domain is something like Distance, Gradient, UV,
--     which specifies an input (ex. Position) and an output corresponding to the domain itself
--
-- This first layer consisting of ... -> A -> A -> B -> B -> ... symbolic compositions
--   A -> A is an input modifier
--   A -> B is a field function
--   B -> B is an output modifier
--   Practically, these describe a shape
--   Formally, these describe a category, which may be the Interval category or some superset of it;
--     it is a category of two objects (A and B) with a morphism connecting them,
--     with some additional stipulation about the presence of A -> A and B -> B arrows that are not the A / B identity morphisms
--     (assuming Haskell's value-equality id function reflects category theory, i.e. t -> t is distinct from T -> T)
--   Preferably, each arrow should be represented as a distinct AST type
--     This allows earlier and more specific errors during typechecking
--   For optimization purposes (ex. minimizing LLVM IR in Rust), it's preferable to represent these in an HList
--     with a corresponding list-chaining function, rather than as a nested set of binary compose actions
--
-- A second layer consisting of symbolic combinator operations over the first layer
--   Union / Intersection / Subtraction
--   Smooth variants of the above
--   Formally, these form a bounded distributive lattice over the set of functions that return real numbers
--     (i.e. distances,) with max as \/ and min as /\, Infinity as top, and -Infinity as bottom.
--     However, more research is needed to determine whether the infinities are mathematically valid bounds
--   They may also form a Set, which is more intuitive w.r.t. Union / Intersection / Subtraction,
--     but this also requires more research for certainty
--   In addition, the symbolic representation implicitly forms a free structure
--   Notably, these operations are evaluated over shapes rather than domains to preserve data for later evaluation
--     The distance domain is evaluated to determine the closer shape subtree, which is returned alongside distance
--     This should also allow for arbitrary decomposition, i.e. splitting shapes at /\ operations as an optimization
--
-- A third layer consisting of symbolic arrow operations over the second layer
--   In effect, the symbolic representation results in the Free Arrow
--   This allows domain outputs to be freely transformed,
--   and ultimately combined into the input of some final output function
--   (Such as shading a color based on position, normal, UV, etc.)
